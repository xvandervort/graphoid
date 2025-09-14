#!/usr/bin/env python3
"""Medical Data Processing with Glang Behaviors

This example shows how the behavior system can handle real-world medical data
with validation, missing value handling, and domain-specific transformations.
"""

from glang.behaviors import BehaviorRegistry, BehaviorPipeline, create_behavior
from glang.execution.values import NumberValue, StringValue, NoneValue, ListValue, HashValue


def create_medical_behaviors():
    """Create domain-specific behaviors for medical data."""
    
    # Blood pressure parser: "120/80" -> average for simplicity
    def parse_bp(value):
        if isinstance(value, StringValue) and "/" in value.value:
            try:
                systolic, diastolic = map(float, value.value.split("/"))
                return NumberValue((systolic + diastolic) / 2)
            except ValueError:
                pass
        return value
    
    # BMI category mapper: numeric BMI -> category number
    def bmi_category(value):
        if isinstance(value, NumberValue):
            bmi = value.value
            if bmi < 18.5:
                return NumberValue(1)  # Underweight
            elif bmi < 25:
                return NumberValue(2)  # Normal
            elif bmi < 30:
                return NumberValue(3)  # Overweight
            else:
                return NumberValue(4)  # Obese
        return value
    
    # Temperature unit converter (assuming Celsius input, keep in range)
    def celsius_to_fahrenheit(value):
        if isinstance(value, NumberValue):
            celsius = value.value
            if 30 <= celsius <= 45:  # Reasonable body temp range in C
                fahrenheit = (celsius * 9/5) + 32
                return NumberValue(fahrenheit)
        return value
    
    # Register behaviors
    registry = BehaviorRegistry()
    registry.register("parse_bp", create_behavior("parse_bp", transform=parse_bp))
    registry.register("bmi_category", create_behavior("bmi_category", transform=bmi_category))
    registry.register("celsius_to_f", create_behavior("celsius_to_f", transform=celsius_to_fahrenheit))
    
    return registry


def demo_vital_signs():
    """Demonstrate processing vital sign measurements."""
    print("=== Vital Signs Processing ===\n")
    
    registry = create_medical_behaviors()
    
    # Temperature pipeline (Fahrenheit, with validation)
    temp_pipeline = BehaviorPipeline(registry)
    temp_pipeline.add("nil_to_zero")
    temp_pipeline.add("validate_range", 95.0, 106.0)  # Normal body temp range (F)
    temp_pipeline.add("round_to_int")
    
    # Heart rate pipeline
    hr_pipeline = BehaviorPipeline(registry)
    hr_pipeline.add("nil_to_zero")
    hr_pipeline.add("validate_range", 40, 120)  # Reasonable heart rate range
    hr_pipeline.add("round_to_int")
    
    # Blood pressure pipeline (parse then validate)
    bp_pipeline = BehaviorPipeline(registry)
    bp_pipeline.add("parse_bp")  # "120/80" -> 100
    bp_pipeline.add("validate_range", 60, 140)  # Reasonable average BP
    bp_pipeline.add("round_to_int")
    
    # Sample vital signs data
    patients_vitals = [
        {
            "id": "P001",
            "temp": NumberValue(98.6),
            "hr": NumberValue(72),
            "bp": StringValue("120/80")
        },
        {
            "id": "P002", 
            "temp": NumberValue(104.5),  # High fever
            "hr": NoneValue(),           # Missing reading
            "bp": StringValue("140/90")  # High BP
        },
        {
            "id": "P003",
            "temp": NumberValue(97.2),
            "hr": NumberValue(200),      # Unrealistic (error)
            "bp": StringValue("invalid") # Bad format
        }
    ]
    
    print("Patient Vital Signs Processing:")
    print("ID     Temp(°F)  Heart Rate  Blood Pressure")
    print("-" * 45)
    
    for patient in patients_vitals:
        # Process each vital sign
        temp = temp_pipeline.apply(patient["temp"])
        hr = hr_pipeline.apply(patient["hr"])
        bp = bp_pipeline.apply(patient["bp"])
        
        print(f"{patient['id']:6} {temp.value:>5}     {hr.value:>5}       {bp.value:>5}")
    
    print()


def demo_patient_data_validation():
    """Demonstrate comprehensive patient data validation."""
    print("=== Patient Data Validation ===\n")
    
    registry = create_medical_behaviors()
    
    # Age validation
    age_pipeline = BehaviorPipeline(registry)
    age_pipeline.add("nil_to_zero")
    age_pipeline.add("validate_range", 0, 120)
    age_pipeline.add("round_to_int")
    
    # BMI calculation and categorization
    bmi_pipeline = BehaviorPipeline(registry)
    bmi_pipeline.add("nil_to_zero") 
    bmi_pipeline.add("validate_range", 10, 50)  # Reasonable BMI range
    bmi_pipeline.add("bmi_category")  # Convert to category number
    
    # Sample patient data
    patient_data = HashValue([
        ("age", NumberValue(45.5)),          # Will be rounded
        ("weight_kg", NumberValue(70)),      # Normal
        ("height_m", NumberValue(1.75)),     # Normal  
        ("bmi", NumberValue(22.9)),          # Normal BMI
        ("emergency_contact", NoneValue()),   # Missing
        ("insurance_id", StringValue("12345"))
    ])
    
    print("Raw Patient Data:")
    for key in ["age", "bmi", "emergency_contact"]:
        value = patient_data.pairs.get(key)
        display = value.to_display_string() if value else "missing"
        print(f"  {key:18}: {display}")
    
    # Apply validations
    age_pipeline.apply_to_hash_value(patient_data, "age")
    bmi_pipeline.apply_to_hash_value(patient_data, "bmi")
    
    print("\nValidated Patient Data:")
    print(f"  age               : {patient_data.pairs.get('age').value}")
    bmi_cat = patient_data.pairs.get('bmi').value
    categories = {1: "Underweight", 2: "Normal", 3: "Overweight", 4: "Obese"}
    print(f"  bmi_category      : {bmi_cat} ({categories.get(bmi_cat, 'Unknown')})")
    
    print()


def demo_clinical_ranges():
    """Demonstrate clinical lab value range validation."""
    print("=== Clinical Lab Values ===\n")
    
    # Different pipelines for different lab tests
    glucose_pipeline = BehaviorPipeline()
    glucose_pipeline.add("nil_to_zero")
    glucose_pipeline.add("validate_range", 70, 200)  # mg/dL normal fasting range (loose)
    
    cholesterol_pipeline = BehaviorPipeline() 
    cholesterol_pipeline.add("nil_to_zero")
    cholesterol_pipeline.add("validate_range", 100, 300)  # mg/dL reasonable range
    
    hemoglobin_pipeline = BehaviorPipeline()
    hemoglobin_pipeline.add("nil_to_zero")
    hemoglobin_pipeline.add("validate_range", 8.0, 18.0)  # g/dL normal range
    
    # Sample lab results with some problematic values
    lab_results = ListValue([
        NumberValue(95),     # Glucose - normal
        NumberValue(500),    # Glucose - extremely high, will be clamped
        NoneValue(),         # Missing value
        NumberValue(-10),    # Invalid negative
        NumberValue(145),    # Normal cholesterol 
        NumberValue(15.2)    # Normal hemoglobin
    ])
    
    print("Lab Value Validation:")
    print("Test            Raw Value  →  Validated")
    print("-" * 40)
    
    test_names = ["Glucose", "Glucose", "Glucose", "Glucose", "Cholesterol", "Hemoglobin"]
    pipelines = [glucose_pipeline, glucose_pipeline, glucose_pipeline, 
                glucose_pipeline, cholesterol_pipeline, hemoglobin_pipeline]
    
    for i, (name, pipeline) in enumerate(zip(test_names, pipelines)):
        raw_value = lab_results.elements[i]
        raw_display = raw_value.to_display_string() if not isinstance(raw_value, NoneValue) else "nil"
        validated = pipeline.apply(raw_value)
        print(f"{name:12}    {raw_display:>8}  →  {validated.value}")
    
    print()


if __name__ == "__main__":
    print("Medical Data Processing with Glang Behaviors\n")
    print("This demonstrates how behaviors can handle:")
    print("- Missing medical readings")
    print("- Invalid/out-of-range values") 
    print("- Domain-specific parsing (blood pressure)")
    print("- Clinical range validation")
    print("- Data categorization (BMI)")
    print("=" * 50)
    print()
    
    demo_vital_signs()
    demo_patient_data_validation()
    demo_clinical_ranges()
    
    print("=== Key Benefits ===")
    print("✓ Graceful handling of missing data")
    print("✓ Automatic validation and range clamping") 
    print("✓ Domain-specific transformations")
    print("✓ Composable validation pipelines")
    print("✓ Type-safe processing")
    print("\nBehaviors make medical data processing robust and reliable!")