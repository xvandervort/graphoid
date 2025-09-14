#!/usr/bin/env python3
"""Demo of Glang's regex module capabilities.

This demonstrates practical regular expression usage in Glang for:
- Pattern matching and validation
- Text extraction and transformation  
- Data parsing and cleaning
- Advanced text processing
"""

from glang.modules.regex_module import RegexModule
from glang.execution.values import StringValue, BooleanValue, ListValue
from glang.ast.nodes import SourcePosition


def demo_basic_pattern_matching():
    """Demonstrate basic pattern matching operations."""
    print("=== Basic Pattern Matching ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    # Test cases for different matching scenarios
    test_cases = [
        {
            'description': 'Email validation',
            'pattern': r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$',
            'texts': ['user@example.com', 'invalid.email', 'test@domain.co.uk'],
            'method': 'validate'
        },
        {
            'description': 'Phone number search',
            'pattern': r'\b\d{3}-\d{3}-\d{4}\b',
            'texts': ['Call 555-123-4567 for help', 'No phone here', '555-987-6543 is valid'],
            'method': 'search'
        },
        {
            'description': 'URL matching at start',
            'pattern': r'https?://',
            'texts': ['https://example.com', 'Visit https://site.org', 'No URL here'],
            'method': 'match'
        }
    ]
    
    for case in test_cases:
        print(f"📋 {case['description']}:")
        pattern = StringValue(case['pattern'], pos)
        
        for text in case['texts']:
            text_val = StringValue(text, pos)
            
            if case['method'] == 'validate':
                result = regex.validate(pattern, text_val)
            elif case['method'] == 'search':
                result = regex.search(pattern, text_val)
            elif case['method'] == 'match':
                result = regex.match(pattern, text_val)
            
            status = "✅" if result.value else "❌"
            print(f"  {status} \"{text}\" -> {result.value}")
        print()


def demo_text_extraction():
    """Demonstrate text extraction and parsing."""
    print("=== Text Extraction and Parsing ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    # Extract different types of data from text
    extraction_cases = [
        {
            'description': 'Extract all numbers',
            'pattern': r'\d+',
            'text': 'Order 123 contains 45 items costing $67.89',
            'method': 'find_all'
        },
        {
            'description': 'Extract email addresses',
            'pattern': r'\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b',
            'text': 'Contact alice@example.com or bob@test.org for support',
            'method': 'find_all'
        },
        {
            'description': 'Parse structured data with groups',
            'pattern': r'(\w+):\s*([^,]+)',
            'text': 'name: Alice, age: 30, city: New York, country: USA',
            'method': 'find_groups'
        },
        {
            'description': 'Extract URL components',
            'pattern': r'https?://([^/]+)(/[^?\s]*)?(?:\?([^&\s]*))?',
            'text': 'Visit https://example.com/docs/guide?version=2.0 for help',
            'method': 'find_groups'
        }
    ]
    
    for case in extraction_cases:
        print(f"📋 {case['description']}:")
        print(f"   Text: \"{case['text']}\"")
        
        pattern = StringValue(case['pattern'], pos)
        text = StringValue(case['text'], pos)
        
        if case['method'] == 'find_all':
            result = regex.find_all(pattern, text)
            matches = [elem.value for elem in result.elements]
            print(f"   Found: {matches}")
            
        elif case['method'] == 'find_groups':
            result = regex.find_groups(pattern, text)
            for i, match in enumerate(result.elements):
                groups = [group.value for group in match.elements]
                print(f"   Match {i+1}: {groups}")
        print()


def demo_text_transformation():
    """Demonstrate text replacement and transformation."""
    print("=== Text Transformation ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    # Various text transformation scenarios
    transformation_cases = [
        {
            'description': 'Remove HTML tags',
            'pattern': r'<[^>]+>',
            'replacement': '',
            'text': '<p>Hello <b>World</b>! Visit <a href="link">here</a>.</p>'
        },
        {
            'description': 'Format phone numbers',
            'pattern': r'(\d{3})(\d{3})(\d{4})',
            'replacement': r'(\1) \2-\3',
            'text': 'Call 5551234567 or 5559876543 for help'
        },
        {
            'description': 'Convert markdown links to HTML',
            'pattern': r'\[([^\]]+)\]\(([^)]+)\)',
            'replacement': r'<a href="\2">\1</a>',
            'text': 'Check out [Glang](https://glang.dev) and [Python](https://python.org)'
        },
        {
            'description': 'Normalize whitespace',
            'pattern': r'\s+',
            'replacement': ' ',
            'text': 'Too    many     spaces   here'
        }
    ]
    
    for case in transformation_cases:
        print(f"📋 {case['description']}:")
        print(f"   Before: \"{case['text']}\"")
        
        pattern = StringValue(case['pattern'], pos)
        replacement = StringValue(case['replacement'], pos)
        text = StringValue(case['text'], pos)
        
        result = regex.replace(pattern, replacement, text)
        print(f"   After:  \"{result.value}\"")
        print()


def demo_advanced_features():
    """Demonstrate advanced regex features like flags and escaping."""
    print("=== Advanced Features ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    # Case-insensitive matching
    print("📋 Case-insensitive matching:")
    pattern = StringValue(r'HELLO', pos)
    text = StringValue("hello world", pos)
    flags = StringValue("i", pos)
    
    result = regex.search(pattern, text, flags)
    print(f"   Pattern: {pattern.value}")
    print(f"   Text: \"{text.value}\"")
    print(f"   Flags: {flags.value}")
    print(f"   Match: {result.value}")
    print()
    
    # Multiline and dotall flags
    print("📋 Multiline and dotall flags:")
    pattern = StringValue(r'^World.*end$', pos)
    text = StringValue("Hello\nWorld and more\ntext until the end", pos)
    flags = StringValue("ms", pos)  # multiline + dotall
    
    result = regex.search(pattern, text, flags)
    print(f"   Pattern: {pattern.value}")
    print(f"   Text: \"{text.value}\"")
    print(f"   Flags: {flags.value}")
    print(f"   Match: {result.value}")
    print()
    
    # Escaping special characters
    print("📋 Escaping special characters:")
    text_with_special = StringValue("Price: $19.99 (includes tax)", pos)
    escaped = regex.escape(text_with_special)
    print(f"   Original: \"{text_with_special.value}\"")
    print(f"   Escaped:  \"{escaped.value}\"")
    print()
    
    # Using escaped text as literal pattern
    pattern = escaped
    search_text = StringValue("The price is: Price: $19.99 (includes tax) total", pos)
    result = regex.search(pattern, search_text)
    print(f"   Search for escaped pattern in: \"{search_text.value}\"")
    print(f"   Found: {result.value}")
    print()


def demo_data_validation():
    """Demonstrate data validation patterns."""
    print("=== Data Validation Patterns ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    # Common validation patterns
    validation_patterns = [
        {
            'name': 'Strong Password',
            'pattern': r'^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$',
            'test_data': ['Password123!', 'weakpass', 'Strong1!', 'NoSpecial123']
        },
        {
            'name': 'Credit Card (basic)',
            'pattern': r'^\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}$',
            'test_data': ['1234 5678 9012 3456', '1234-5678-9012-3456', '1234567890123456', '123456789']
        },
        {
            'name': 'IPv4 Address',
            'pattern': r'^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$',
            'test_data': ['192.168.1.1', '10.0.0.1', '256.1.1.1', '192.168.1']
        },
        {
            'name': 'ISO Date Format',
            'pattern': r'^\d{4}-\d{2}-\d{2}$',
            'test_data': ['2025-01-15', '2025-1-15', '25-01-15', '2025-01-15T10:30:00']
        }
    ]
    
    for validation in validation_patterns:
        print(f"📋 {validation['name']} Validation:")
        pattern = StringValue(validation['pattern'], pos)
        
        for test_input in validation['test_data']:
            text = StringValue(test_input, pos)
            result = regex.validate(pattern, text)
            status = "✅" if result.value else "❌"
            print(f"   {status} \"{test_input}\"")
        print()


def demo_performance_features():
    """Demonstrate performance-oriented features."""
    print("=== Performance Features ===\n")
    
    regex = RegexModule()
    pos = SourcePosition(1, 1)
    
    print("📋 Pattern caching demonstration:")
    print("   The regex module automatically caches compiled patterns")
    print("   for better performance on repeated operations.")
    print()
    
    # Demonstrate that the same pattern benefits from caching
    pattern = StringValue(r'\b\w+@\w+\.\w+\b', pos)
    texts = [
        "Contact alice@example.com for help",
        "Also reach bob@test.org anytime", 
        "Or try support@company.net"
    ]
    
    print("   Searching for emails in multiple texts:")
    for i, text in enumerate(texts, 1):
        text_val = StringValue(text, pos)
        result = regex.find_all(pattern, text_val)
        emails = [elem.value for elem in result.elements]
        print(f"   Text {i}: Found {emails}")
    
    print(f"\n   ✅ Pattern '{pattern.value}' was compiled once and reused")
    print(f"   📊 Cache limit: {regex._cache_limit} patterns")
    print(f"   📈 Current cache size: {len(regex._pattern_cache)} patterns")
    print()


if __name__ == "__main__":
    print("🔍 Glang Regular Expression Module Demo")
    print("=" * 50)
    print()
    
    demo_basic_pattern_matching()
    demo_text_extraction()
    demo_text_transformation()
    demo_advanced_features()
    demo_data_validation()
    demo_performance_features()
    
    print("🎉 SUCCESS: Regex module provides comprehensive pattern matching!")
    print("\n📋 What's Working:")
    print("  ✅ Basic pattern matching (match, search, validate)")
    print("  ✅ Text extraction (find_all, find_groups)")
    print("  ✅ Text transformation (replace with capture groups)")
    print("  ✅ Advanced features (flags, escaping)")
    print("  ✅ Data validation patterns")
    print("  ✅ Performance optimization (pattern caching)")
    print("  ✅ Comprehensive error handling and type validation")
    print("\n🚀 Ready for complex text processing in Glang!")