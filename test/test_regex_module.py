"""Tests for the regex module."""

import pytest
from glang.modules.regex_module import RegexModule, create_regex_module_namespace
from glang.execution.values import StringValue, BooleanValue, ListValue, NumberValue
from glang.ast.nodes import SourcePosition


class TestRegexModule:
    """Test the regex module functionality."""
    
    def setup_method(self):
        self.regex = RegexModule()
        self.pos = SourcePosition(1, 1)
    
    def test_match_success(self):
        """Test successful pattern matching at start of string."""
        pattern = StringValue(r"Hello", self.pos)
        text = StringValue("Hello World", self.pos)
        
        result = self.regex.match(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_match_failure(self):
        """Test failed pattern matching (pattern not at start)."""
        pattern = StringValue(r"World", self.pos)
        text = StringValue("Hello World", self.pos)
        
        result = self.regex.match(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is False
    
    def test_search_success(self):
        """Test successful pattern search anywhere in string."""
        pattern = StringValue(r"World", self.pos)
        text = StringValue("Hello World", self.pos)
        
        result = self.regex.search(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_search_failure(self):
        """Test failed pattern search."""
        pattern = StringValue(r"Python", self.pos)
        text = StringValue("Hello World", self.pos)
        
        result = self.regex.search(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is False
    
    def test_find_all(self):
        """Test finding all matches of a pattern."""
        pattern = StringValue(r"\d+", self.pos)
        text = StringValue("I have 42 apples and 17 oranges", self.pos)
        
        result = self.regex.find_all(pattern, text)
        
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2
        assert result.elements[0].value == "42"
        assert result.elements[1].value == "17"
        assert result.constraint == "string"
    
    def test_find_groups(self):
        """Test finding matches with capture groups."""
        pattern = StringValue(r"(\w+)@(\w+\.\w+)", self.pos)
        text = StringValue("Email alice@example.com and bob@test.org", self.pos)
        
        result = self.regex.find_groups(pattern, text)
        
        assert isinstance(result, ListValue)
        assert len(result.elements) == 2  # Two email matches
        
        # First match: alice@example.com
        first_match = result.elements[0]
        assert isinstance(first_match, ListValue)
        assert len(first_match.elements) == 2
        assert first_match.elements[0].value == "alice"
        assert first_match.elements[1].value == "example.com"
        
        # Second match: bob@test.org
        second_match = result.elements[1]
        assert isinstance(second_match, ListValue)
        assert len(second_match.elements) == 2
        assert second_match.elements[0].value == "bob"
        assert second_match.elements[1].value == "test.org"
    
    def test_replace_simple(self):
        """Test simple string replacement."""
        pattern = StringValue(r"\d+", self.pos)
        replacement = StringValue("X", self.pos)
        text = StringValue("I have 42 apples and 17 oranges", self.pos)
        
        result = self.regex.replace(pattern, replacement, text)
        
        assert isinstance(result, StringValue)
        assert result.value == "I have X apples and X oranges"
    
    def test_replace_with_groups(self):
        """Test replacement using capture groups."""
        pattern = StringValue(r"(\w+)@(\w+\.\w+)", self.pos)
        replacement = StringValue(r"\1 at \2", self.pos)
        text = StringValue("Contact alice@example.com for help", self.pos)
        
        result = self.regex.replace(pattern, replacement, text)
        
        assert isinstance(result, StringValue)
        assert result.value == "Contact alice at example.com for help"
    
    def test_split(self):
        """Test splitting text by regex pattern."""
        pattern = StringValue(r"[,;:]", self.pos)
        text = StringValue("apple,banana;cherry:date", self.pos)
        
        result = self.regex.split(pattern, text)
        
        assert isinstance(result, ListValue)
        assert len(result.elements) == 4
        assert result.elements[0].value == "apple"
        assert result.elements[1].value == "banana"
        assert result.elements[2].value == "cherry"
        assert result.elements[3].value == "date"
        assert result.constraint == "string"
    
    def test_validate_success(self):
        """Test successful full pattern validation."""
        pattern = StringValue(r"\d{4}-\d{2}-\d{2}", self.pos)
        text = StringValue("2025-01-15", self.pos)
        
        result = self.regex.validate(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_validate_failure(self):
        """Test failed full pattern validation."""
        pattern = StringValue(r"\d{4}-\d{2}-\d{2}", self.pos)
        text = StringValue("2025-01-15 10:30", self.pos)  # Extra text
        
        result = self.regex.validate(pattern, text)
        
        assert isinstance(result, BooleanValue)
        assert result.value is False
    
    def test_escape(self):
        """Test escaping special regex characters."""
        text = StringValue("Hello (world) [test]", self.pos)
        
        result = self.regex.escape(text)
        
        assert isinstance(result, StringValue)
        assert result.value == r"Hello\ \(world\)\ \[test\]"
    
    def test_flags_ignore_case(self):
        """Test regex with ignore case flag."""
        pattern = StringValue(r"HELLO", self.pos)
        text = StringValue("hello world", self.pos)
        flags = StringValue("i", self.pos)
        
        result = self.regex.search(pattern, text, flags)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_flags_multiline(self):
        """Test regex with multiline flag."""
        pattern = StringValue(r"^World", self.pos)
        text = StringValue("Hello\nWorld", self.pos)
        flags = StringValue("m", self.pos)
        
        result = self.regex.search(pattern, text, flags)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_flags_dotall(self):
        """Test regex with dotall flag."""
        pattern = StringValue(r"Hello.*World", self.pos)
        text = StringValue("Hello\nWorld", self.pos)
        flags = StringValue("s", self.pos)
        
        result = self.regex.search(pattern, text, flags)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_multiple_flags(self):
        """Test regex with multiple flags."""
        pattern = StringValue(r"^HELLO.*world", self.pos)
        text = StringValue("hello\nworld", self.pos)
        flags = StringValue("ims", self.pos)  # ignore case + multiline + dotall
        
        result = self.regex.search(pattern, text, flags)
        
        assert isinstance(result, BooleanValue)
        assert result.value is True
    
    def test_invalid_pattern(self):
        """Test error handling for invalid regex pattern."""
        pattern = StringValue(r"[invalid", self.pos)  # Unclosed bracket
        text = StringValue("test", self.pos)
        
        with pytest.raises(ValueError, match="Invalid regex pattern"):
            self.regex.match(pattern, text)
    
    def test_invalid_flag(self):
        """Test error handling for invalid regex flag."""
        pattern = StringValue(r"test", self.pos)
        text = StringValue("test", self.pos)
        flags = StringValue("z", self.pos)  # Invalid flag
        
        with pytest.raises(ValueError, match="Unknown regex flag"):
            self.regex.match(pattern, text, flags)
    
    def test_type_validation(self):
        """Test type validation for method parameters."""
        pattern = NumberValue(123, self.pos)  # Wrong type
        text = StringValue("test", self.pos)
        
        with pytest.raises(TypeError, match="Pattern must be string"):
            self.regex.match(pattern, text)


class TestRegexModuleIntegration:
    """Test regex module integration with Glang module system."""
    
    def test_module_namespace_creation(self):
        """Test that module namespace is created correctly."""
        namespace = create_regex_module_namespace()
        
        assert namespace.filename == "regex"
        
        # Check that all expected functions are available
        expected_functions = [
            'match', 'search', 'find_all', 'find_groups', 
            'replace', 'split', 'validate', 'escape'
        ]
        
        for func_name in expected_functions:
            assert namespace.get_symbol(func_name) is not None, f"Missing function: {func_name}"


class TestRegexPracticalExamples:
    """Test practical regex usage examples."""
    
    def setup_method(self):
        self.regex = RegexModule()
        self.pos = SourcePosition(1, 1)
    
    def test_email_validation(self):
        """Test email validation with regex."""
        pattern = StringValue(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", self.pos)
        
        valid_email = StringValue("user@example.com", self.pos)
        invalid_email = StringValue("invalid.email", self.pos)
        
        assert self.regex.validate(pattern, valid_email).value is True
        assert self.regex.validate(pattern, invalid_email).value is False
    
    def test_phone_number_extraction(self):
        """Test extracting phone numbers from text."""
        pattern = StringValue(r"\b\d{3}-\d{3}-\d{4}\b", self.pos)
        text = StringValue("Call 555-123-4567 or 555-987-6543 for help", self.pos)
        
        result = self.regex.find_all(pattern, text)
        
        assert len(result.elements) == 2
        assert result.elements[0].value == "555-123-4567"
        assert result.elements[1].value == "555-987-6543"
    
    def test_html_tag_removal(self):
        """Test removing HTML tags from text."""
        pattern = StringValue(r"<[^>]+>", self.pos)
        replacement = StringValue("", self.pos)
        text = StringValue("<p>Hello <b>World</b></p>", self.pos)
        
        result = self.regex.replace(pattern, replacement, text)
        
        assert result.value == "Hello World"
    
    def test_csv_parsing(self):
        """Test parsing CSV-like data with regex."""
        pattern = StringValue(r",(?=(?:[^\"]*\"[^\"]*\")*[^\"]*$)", self.pos)
        text = StringValue('name,age,"city, state",country', self.pos)
        
        result = self.regex.split(pattern, text)
        
        assert len(result.elements) == 4
        assert result.elements[0].value == "name"
        assert result.elements[1].value == "age"
        assert result.elements[2].value == '"city, state"'
        assert result.elements[3].value == "country"
    
    def test_password_strength_validation(self):
        """Test password strength validation."""
        # Password must have: 8+ chars, uppercase, lowercase, digit, special char
        pattern = StringValue(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$", self.pos)
        
        strong_password = StringValue("MyPass123!", self.pos)
        weak_password = StringValue("password", self.pos)
        
        assert self.regex.validate(pattern, strong_password).value is True
        assert self.regex.validate(pattern, weak_password).value is False
    
    def test_url_extraction_with_groups(self):
        """Test extracting URL components with capture groups."""
        pattern = StringValue(r"https?://([^/]+)(/[^?\s]*)?(?:\?([^&\s]*))?", self.pos)
        text = StringValue("Visit https://example.com/path/page?param=value for info", self.pos)
        
        result = self.regex.find_groups(pattern, text)
        
        assert len(result.elements) == 1
        url_parts = result.elements[0]
        assert len(url_parts.elements) == 3
        assert url_parts.elements[0].value == "example.com"  # domain
        assert url_parts.elements[1].value == "/path/page"   # path
        assert url_parts.elements[2].value == "param=value"  # query
    
    def test_markdown_link_transformation(self):
        """Test transforming markdown links to HTML."""
        pattern = StringValue(r"\[([^\]]+)\]\(([^)]+)\)", self.pos)
        replacement = StringValue(r'<a href="\2">\1</a>', self.pos)
        text = StringValue("Check out [Glang](https://glang.dev) for more info", self.pos)
        
        result = self.regex.replace(pattern, replacement, text)
        
        assert result.value == 'Check out <a href="https://glang.dev">Glang</a> for more info'