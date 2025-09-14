#!/usr/bin/env python3
"""Configuration Management with Glang Behaviors

This example shows how behaviors can normalize, validate, and transform
application configuration data from various sources.
"""

from glang.behaviors import BehaviorRegistry, BehaviorPipeline, create_behavior
from glang.execution.values import NumberValue, StringValue, BooleanValue, NoneValue, HashValue


def create_config_behaviors():
    """Create behaviors for configuration processing."""
    
    # Environment variable style normalizer
    def env_normalize(value):
        if isinstance(value, StringValue):
            normalized = value.value.strip().lower()
            # Common environment variable patterns
            if normalized in ['true', 'yes', '1', 'on', 'enabled']:
                return BooleanValue(True)
            elif normalized in ['false', 'no', '0', 'off', 'disabled']:
                return BooleanValue(False)
            # Try to parse as number
            try:
                num = float(normalized)
                return NumberValue(num)
            except ValueError:
                pass
            # Return normalized string
            return StringValue(normalized)
        return value
    
    # URL validator/normalizer
    def normalize_url(value):
        if isinstance(value, StringValue):
            url = value.value.strip()
            if not url.startswith(('http://', 'https://')):
                url = 'http://' + url
            if url.endswith('/'):
                url = url[:-1]
            return StringValue(url)
        return value
    
    # Log level mapper
    def normalize_log_level(value):
        if isinstance(value, StringValue):
            level = value.value.upper()
            level_map = {
                'DEBUG': 1, 'INFO': 2, 'WARN': 3, 'WARNING': 3,
                'ERROR': 4, 'CRITICAL': 5, 'FATAL': 5
            }
            if level in level_map:
                return NumberValue(level_map[level])
        return value
    
    registry = BehaviorRegistry()
    registry.register("env_normalize", create_behavior("env_normalize", transform=env_normalize))
    registry.register("normalize_url", create_behavior("normalize_url", transform=normalize_url))
    registry.register("normalize_log_level", create_behavior("normalize_log_level", transform=normalize_log_level))
    
    return registry


def demo_server_config():
    """Demonstrate server configuration processing."""
    print("=== Server Configuration ===\n")
    
    registry = create_config_behaviors()
    
    # Port validation pipeline
    port_pipeline = BehaviorPipeline(registry)
    port_pipeline.add("env_normalize")  # Parse string numbers
    port_pipeline.add("validate_range", 1024, 65535)  # Valid port range
    port_pipeline.add("round_to_int")
    
    # URL normalization pipeline
    url_pipeline = BehaviorPipeline(registry)
    url_pipeline.add("normalize_url")
    
    # Log level pipeline
    log_pipeline = BehaviorPipeline(registry)
    log_pipeline.add("normalize_log_level")
    
    # Boolean pipeline for flags
    bool_pipeline = BehaviorPipeline(registry)
    bool_pipeline.add("env_normalize")  # Handles various boolean representations
    
    # Raw configuration (as might come from environment variables)
    config = HashValue([
        ("port", StringValue("8080")),           # String number
        ("admin_port", StringValue("80")),       # Too low
        ("debug", StringValue("true")),          # String boolean
        ("log_level", StringValue("warning")),   # String log level  
        ("api_url", StringValue("api.example.com/")), # Incomplete URL
        ("database_url", StringValue("localhost:5432")), # No protocol
        ("ssl_enabled", StringValue("yes")),     # String boolean
        ("max_connections", StringValue("1000")), # String number
    ])
    
    print("Raw Configuration (from environment variables):")
    for key in ["port", "admin_port", "debug", "log_level", "api_url", "ssl_enabled"]:
        value = config.pairs.get(key)
        print(f"  {key:15}: '{value.value}'")
    
    # Apply appropriate pipelines
    port_pipeline.apply_to_hash_value(config, "port")
    port_pipeline.apply_to_hash_value(config, "admin_port")
    bool_pipeline.apply_to_hash_value(config, "debug")
    log_pipeline.apply_to_hash_value(config, "log_level")
    url_pipeline.apply_to_hash_value(config, "api_url")
    url_pipeline.apply_to_hash_value(config, "database_url")
    bool_pipeline.apply_to_hash_value(config, "ssl_enabled")
    port_pipeline.apply_to_hash_value(config, "max_connections")
    
    print("\nProcessed Configuration:")
    config_display = {
        "port": f"{config.pairs.get('port').value} (validated port)",
        "admin_port": f"{config.pairs.get('admin_port').value} (clamped to minimum)",
        "debug": f"{config.pairs.get('debug').value} (parsed boolean)",
        "log_level": f"{config.pairs.get('log_level').value} (3=WARNING)",
        "api_url": f"'{config.pairs.get('api_url').value}' (normalized URL)",
        "ssl_enabled": f"{config.pairs.get('ssl_enabled').value} (parsed boolean)",
    }
    
    for key, display in config_display.items():
        print(f"  {key:15}: {display}")
    
    print()


def demo_database_config():
    """Demonstrate database configuration with connection pooling."""
    print("=== Database Configuration ===\n")
    
    # Connection pool settings pipeline
    pool_pipeline = BehaviorPipeline()
    pool_pipeline.add("nil_to_zero")
    pool_pipeline.add("validate_range", 1, 100)  # Reasonable pool size
    pool_pipeline.add("round_to_int")
    
    # Timeout pipeline
    timeout_pipeline = BehaviorPipeline() 
    timeout_pipeline.add("nil_to_zero")
    timeout_pipeline.add("validate_range", 1, 300)  # 1-300 seconds
    timeout_pipeline.add("round_to_int")
    
    # Database configuration with some problematic values
    db_config = HashValue([
        ("host", StringValue("localhost")),
        ("port", NumberValue(5432)),
        ("database", StringValue("myapp")),
        ("pool_size", NoneValue()),              # Not configured
        ("max_overflow", NumberValue(200)),      # Too high
        ("connection_timeout", NumberValue(0.5)), # Too low
        ("retry_attempts", NumberValue(-5)),      # Negative
    ])
    
    print("Database Configuration Processing:")
    print("Setting              Raw Value  →  Processed")
    print("-" * 50)
    
    # Show original values
    settings = ["pool_size", "max_overflow", "connection_timeout", "retry_attempts"]
    originals = []
    for setting in settings:
        value = db_config.pairs.get(setting)
        display = value.to_display_string() if value and not isinstance(value, NoneValue) else "nil"
        originals.append(display)
        print(f"{setting:18}  {display:>8}  →  ", end="")
        
        # Apply appropriate pipeline
        if "timeout" in setting:
            timeout_pipeline.apply_to_hash_value(db_config, setting)
        else:
            pool_pipeline.apply_to_hash_value(db_config, setting)
        
        new_value = db_config.pairs.get(setting)
        print(f"{new_value.value}")
    
    print()


def demo_feature_flags():
    """Demonstrate feature flag configuration."""
    print("=== Feature Flags Configuration ===\n")
    
    registry = create_config_behaviors()
    
    # Feature flag pipeline (normalize various boolean representations)
    flag_pipeline = BehaviorPipeline(registry)
    flag_pipeline.add("env_normalize")  # Handles true/false/1/0/yes/no etc.
    
    # Feature flags from various sources
    feature_flags = HashValue([
        ("new_ui", StringValue("true")),
        ("beta_features", StringValue("1")),
        ("debug_mode", StringValue("yes")),
        ("legacy_api", StringValue("false")),
        ("analytics", StringValue("on")),
        ("cache_enabled", StringValue("disabled")),
        ("experimental", StringValue("maybe")),  # Invalid boolean
    ])
    
    print("Feature Flags Processing:")
    print("Flag                Raw Value    →  Enabled")
    print("-" * 42)
    
    flags = ["new_ui", "beta_features", "debug_mode", "legacy_api", 
             "analytics", "cache_enabled", "experimental"]
    
    for flag in flags:
        original = feature_flags.pairs.get(flag)
        original_display = f"'{original.value}'"
        
        flag_pipeline.apply_to_hash_value(feature_flags, flag)
        processed = feature_flags.pairs.get(flag)
        
        # Display result
        if isinstance(processed, BooleanValue):
            result = "Yes" if processed.value else "No"
        else:
            result = f"Invalid ({processed.to_display_string()})"
            
        print(f"{flag:15}   {original_display:>10}  →  {result}")
    
    print()


if __name__ == "__main__":
    print("Configuration Management with Glang Behaviors\n")
    print("This demonstrates how behaviors can:")
    print("- Normalize environment variables to proper types")
    print("- Validate configuration ranges") 
    print("- Handle missing configuration gracefully")
    print("- Parse various boolean representations")
    print("- Normalize URLs and other structured data")
    print("=" * 55)
    print()
    
    demo_server_config()
    demo_database_config()
    demo_feature_flags()
    
    print("=== Key Benefits for Configuration ===")
    print("✓ Robust parsing of environment variables")
    print("✓ Type-safe configuration processing")
    print("✓ Automatic validation and range checking")
    print("✓ Graceful handling of missing values")
    print("✓ Consistent normalization across config sources")
    print("\nBehaviors eliminate configuration bugs before they happen!")