"""Test enhanced string methods functionality."""

import pytest
from glang.execution.values import StringValue, BooleanValue, ListValue
from glang.execution.executor import ASTExecutor, ExecutionContext
from glang.semantic.analyzer import SemanticAnalyzer
from glang.semantic.symbol_table import SymbolTable
from glang.parser.ast_parser import ASTParser
from glang.lexer.tokenizer import Tokenizer


class TestEnhancedStringMethods:
    """Test enhanced string method functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.tokenizer = Tokenizer()
        self.parser = ASTParser()
        self.analyzer = SemanticAnalyzer()
        self.symbol_table = SymbolTable()
        self.context = ExecutionContext(self.symbol_table)
        self.executor = ASTExecutor(self.context)
    
    def execute_string_method(self, string_value, method_name, *args):
        """Helper to execute string methods directly."""
        string_val = StringValue(string_value)
        arg_values = [StringValue(str(arg)) for arg in args]
        return self.executor._dispatch_string_method(string_val, method_name, arg_values, None)
    
    def test_contains_any_digits(self):
        """Test unified contains('any', ...) with digits pattern."""
        result = self.execute_string_method("Hello123", "contains", "any", "digits")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("Hello", "contains", "any", "digits")
        assert result.value is False
    
    def test_contains_any_letters(self):
        """Test unified contains('any', ...) with letters pattern."""
        result = self.execute_string_method("123abc", "contains", "any", "letters")
        assert result.value is True
        
        result = self.execute_string_method("123!@#", "contains", "any", "letters")
        assert result.value is False
    
    def test_contains_any_uppercase(self):
        """Test unified contains('any', ...) with uppercase pattern."""
        result = self.execute_string_method("Hello", "contains", "any", "uppercase")
        assert result.value is True
        
        result = self.execute_string_method("hello", "contains", "any", "uppercase")
        assert result.value is False
    
    def test_contains_any_lowercase(self):
        """Test unified contains('any', ...) with lowercase pattern."""
        result = self.execute_string_method("Hello", "contains", "any", "lowercase")
        assert result.value is True
        
        result = self.execute_string_method("HELLO", "contains", "any", "lowercase")
        assert result.value is False
    
    def test_contains_any_spaces(self):
        """Test unified contains('any', ...) with spaces pattern."""
        result = self.execute_string_method("Hello World", "contains", "any", "spaces")
        assert result.value is True
        
        result = self.execute_string_method("HelloWorld", "contains", "any", "spaces")
        assert result.value is False
    
    def test_contains_any_punctuation(self):
        """Test unified contains('any', ...) with punctuation pattern."""
        result = self.execute_string_method("Hello!", "contains", "any", "punctuation")
        assert result.value is True
        
        result = self.execute_string_method("Hello", "contains", "any", "punctuation")
        assert result.value is False
    
    def test_contains_all_multiple_patterns(self):
        """Test unified contains('all', ...) with multiple patterns."""
        result = self.execute_string_method("Hello123!", "contains", "all", "letters", "digits", "punctuation")
        assert result.value is True
        
        result = self.execute_string_method("Hello123", "contains", "all", "letters", "digits", "punctuation")
        assert result.value is False
    
    def test_contains_only_single_pattern(self):
        """Test unified contains('only', ...) with single pattern."""
        result = self.execute_string_method("123", "contains", "only", "digits")
        assert result.value is True
        
        result = self.execute_string_method("123a", "contains", "only", "digits")
        assert result.value is False
    
    def test_contains_only_multiple_patterns(self):
        """Test unified contains('only', ...) with multiple patterns."""
        result = self.execute_string_method("Hello World", "contains", "only", "letters", "spaces")
        assert result.value is True
        
        result = self.execute_string_method("Hello World!", "contains", "only", "letters", "spaces")
        assert result.value is False
    
    def test_extract_numbers(self):
        """Test unified extract('numbers') method."""
        result = self.execute_string_method("I have 5 apples and 3.14 pies", "extract", "numbers")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "5"
        assert result.elements[1].value == "3.14"
    
    def test_extract_numbers_with_negatives(self):
        """Test unified extract('numbers') with negative numbers."""
        result = self.execute_string_method("Temperature is -5.2 degrees", "extract", "numbers")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 1
        assert result.elements[0].value == "-5.2"
    
    def test_extract_words(self):
        """Test unified extract('words') method."""
        result = self.execute_string_method("Hello, World! 123", "extract", "words")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "Hello"
        assert result.elements[1].value == "World"
    
    def test_extract_emails(self):
        """Test unified extract('emails') method."""
        result = self.execute_string_method("Contact us at support@example.com or sales@company.org", "extract", "emails")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "support@example.com"
        assert result.elements[1].value == "sales@company.org"
    
    def test_is_email_valid(self):
        """Test is_email with valid emails."""
        result = self.execute_string_method("user@example.com", "is_email")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("test.email+tag@domain.co.uk", "is_email")
        assert result.value is True
    
    def test_is_email_invalid(self):
        """Test is_email with invalid emails."""
        result = self.execute_string_method("invalid-email", "is_email")
        assert result.value is False
        
        result = self.execute_string_method("@example.com", "is_email")
        assert result.value is False
        
        result = self.execute_string_method("user@", "is_email")
        assert result.value is False
    
    def test_is_number_valid(self):
        """Test is_number with valid numbers."""
        result = self.execute_string_method("123", "is_number")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("3.14", "is_number")
        assert result.value is True
        
        result = self.execute_string_method("-42.5", "is_number")
        assert result.value is True
    
    def test_is_number_invalid(self):
        """Test is_number with invalid numbers."""
        result = self.execute_string_method("abc", "is_number")
        assert result.value is False
        
        result = self.execute_string_method("12.34.56", "is_number")
        assert result.value is False
    
    def test_is_url_valid(self):
        """Test is_url with valid URLs."""
        result = self.execute_string_method("https://example.com", "is_url")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("http://test.org/path", "is_url")
        assert result.value is True
        
        result = self.execute_string_method("ftp://files.example.com", "is_url")
        assert result.value is True
    
    def test_is_url_invalid(self):
        """Test is_url with invalid URLs."""
        result = self.execute_string_method("not-a-url", "is_url")
        assert result.value is False
        
        result = self.execute_string_method("example.com", "is_url")
        assert result.value is False
    
    def test_split_on_any(self):
        """Test split_on_any method."""
        result = self.execute_string_method("apple,banana;orange|grape", "split_on_any", ",;|")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 4
        assert result.elements[0].value == "apple"
        assert result.elements[1].value == "banana"
        assert result.elements[2].value == "orange"
        assert result.elements[3].value == "grape"
    
    def test_split_on_any_with_spaces(self):
        """Test split_on_any with multiple consecutive delimiters."""
        result = self.execute_string_method("a,,b;;c", "split_on_any", ",;")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 3  # Empty strings should be filtered out
        assert result.elements[0].value == "a"
        assert result.elements[1].value == "b"
        assert result.elements[2].value == "c"
    
    def test_find_all_alias(self):
        """Test that find_all and findAll both work."""
        # Test new name
        result = self.execute_string_method("test123test456", "find_all", r"\d+")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "123"
        assert result.elements[1].value == "456"
        
        # Test old name (should still work for backward compatibility)
        result = self.execute_string_method("test123test456", "findAll", r"\d+")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "123"
        assert result.elements[1].value == "456"
    
    def test_contains_backward_compatibility(self):
        """Test that old substring contains() still works."""
        # Old single-argument contains for substring search
        result = self.execute_string_method("Hello World", "contains", "World")
        assert isinstance(result, BooleanValue)
        assert result.value is True
        
        result = self.execute_string_method("Hello World", "contains", "xyz")
        assert result.value is False
    
    def test_contains_unified_interface_comprehensive(self):
        """Test comprehensive scenarios with unified contains interface."""
        # Test 'any' mode with various patterns
        result = self.execute_string_method("Hello123!", "contains", "any", "symbols")
        assert result.value is True
        
        result = self.execute_string_method("Hello123!", "contains", "any", "alphanumeric")
        assert result.value is True
        
        # Test 'all' mode - should have all specified patterns
        result = self.execute_string_method("Hello123!", "contains", "all", "letters", "digits", "symbols")
        assert result.value is True
        
        # Test 'only' mode - should contain only specified character types
        result = self.execute_string_method("ABC123", "contains", "only", "letters", "digits")
        assert result.value is True
        
        result = self.execute_string_method("ABC123!", "contains", "only", "letters", "digits")
        assert result.value is False  # Contains punctuation too
    
    def test_extract_comprehensive(self):
        """Test comprehensive scenarios with unified extract interface."""
        # Test complex text with multiple pattern types
        text = "Contact John at john@example.com or call 555-123-4567 for details about order #12345"
        
        # Extract numbers
        result = self.execute_string_method(text, "extract", "numbers")
        assert isinstance(result, ListValue)
        assert len(result.elements) >= 3  # 555, 123, 4567, 12345
        
        # Extract words  
        result = self.execute_string_method(text, "extract", "words")
        assert isinstance(result, ListValue)
        assert len(result.elements) >= 8  # Contact, John, at, john, example, com, etc.
        
        # Extract emails
        result = self.execute_string_method(text, "extract", "emails")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 1
        assert result.elements[0].value == "john@example.com"
    
    def test_edge_cases(self):
        """Test edge cases and error conditions."""
        # Empty string
        result = self.execute_string_method("", "contains", "any", "letters")
        assert result.value is False
        
        result = self.execute_string_method("", "extract", "numbers")
        assert isinstance(result, ListValue)
        assert len(result.elements) == 0
        
        # Single character strings
        result = self.execute_string_method("a", "contains", "only", "letters")
        assert result.value is True
        
        result = self.execute_string_method("1", "contains", "only", "digits")
        assert result.value is True