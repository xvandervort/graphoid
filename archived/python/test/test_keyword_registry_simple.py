"""Simple tests for keyword registry functionality."""

import pytest
from src.glang.language.keyword_registry import (
    LanguageKeywordRegistry, 
    KeywordDefinition, 
    KeywordCategory
)


class TestKeywordDefinition:
    """Test KeywordDefinition class."""
    
    def test_basic_keyword_definition(self):
        """Test basic keyword definition creation."""
        definition = KeywordDefinition(
            keyword="test",
            category=KeywordCategory.TYPE,
            parser_method="parse_test",
            description="Test keyword",
            syntax_example="test example"
        )
        
        assert definition.keyword == "test"
        assert definition.category == KeywordCategory.TYPE
        assert definition.parser_method == "parse_test"
        assert definition.description == "Test keyword"
        assert definition.syntax_example == "test example"
    
    def test_keyword_definition_with_token_type(self):
        """Test keyword definition with custom token type."""
        definition = KeywordDefinition(
            keyword="custom",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_custom",
            description="Custom keyword",
            syntax_example="custom example",
            token_type_name="CUSTOM_TOKEN"
        )
        
        assert definition.token_type_name == "CUSTOM_TOKEN"
        assert definition.get_token_type_name() == "CUSTOM_TOKEN"
    
    def test_keyword_definition_auto_token_type(self):
        """Test automatic token type name generation."""
        definition = KeywordDefinition(
            keyword="import",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_import",
            description="Import statement",
            syntax_example="import module"
        )
        
        assert definition.token_type_name is None
        assert definition.get_token_type_name() == "IMPORT"
    
    def test_keyword_definition_with_aliases(self):
        """Test keyword definition with aliases."""
        definition = KeywordDefinition(
            keyword="function",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_function",
            description="Function definition",
            syntax_example="function name() {}",
            aliases=["func", "fn"]
        )
        
        assert definition.aliases == ["func", "fn"]


class TestKeywordCategory:
    """Test KeywordCategory enum."""
    
    def test_keyword_categories_exist(self):
        """Test that expected keyword categories exist."""
        assert KeywordCategory.TYPE.value == "type"
        assert KeywordCategory.STATEMENT.value == "statement"
        assert KeywordCategory.LITERAL.value == "literal"
    
    def test_category_enum_values(self):
        """Test category enum values."""
        categories = list(KeywordCategory)
        
        assert len(categories) >= 3
        assert KeywordCategory.TYPE in categories
        assert KeywordCategory.STATEMENT in categories
        assert KeywordCategory.LITERAL in categories


class TestLanguageKeywordRegistry:
    """Test LanguageKeywordRegistry class."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.registry = LanguageKeywordRegistry()
    
    def test_registry_initialization(self):
        """Test that registry initializes properly."""
        # Should be able to create without errors
        assert isinstance(self.registry, LanguageKeywordRegistry)
    
    def test_registry_has_methods(self):
        """Test that registry has expected methods."""
        # Check for expected methods
        assert hasattr(self.registry, '_keywords')
        assert hasattr(self.registry, '_aliases')
        
        # Should be dictionaries
        assert isinstance(self.registry._keywords, dict)
        assert isinstance(self.registry._aliases, dict)


class TestKeywordDefinitionEdgeCases:
    """Test edge cases for KeywordDefinition."""
    
    def test_empty_keyword(self):
        """Test keyword definition with empty keyword."""
        definition = KeywordDefinition(
            keyword="",
            category=KeywordCategory.TYPE,
            parser_method="parse_empty",
            description="Empty keyword",
            syntax_example=""
        )
        
        assert definition.keyword == ""
        assert definition.get_token_type_name() == ""
    
    def test_lowercase_keyword_uppercase_token(self):
        """Test that lowercase keywords produce uppercase tokens."""
        definition = KeywordDefinition(
            keyword="lowercase",
            category=KeywordCategory.TYPE,
            parser_method="parse_lowercase",
            description="Lowercase keyword",
            syntax_example="lowercase"
        )
        
        assert definition.get_token_type_name() == "LOWERCASE"
    
    def test_keyword_with_special_characters(self):
        """Test keyword with special characters."""
        definition = KeywordDefinition(
            keyword="key-word",
            category=KeywordCategory.TYPE,
            parser_method="parse_key_word",
            description="Hyphenated keyword",
            syntax_example="key-word"
        )
        
        assert definition.keyword == "key-word"
        assert definition.get_token_type_name() == "KEY-WORD"
    
    def test_optional_fields_none(self):
        """Test that optional fields can be None."""
        definition = KeywordDefinition(
            keyword="minimal",
            category=KeywordCategory.TYPE,
            parser_method="parse_minimal",
            description="Minimal keyword",
            syntax_example="minimal",
            token_type_name=None,
            aliases=None
        )
        
        assert definition.token_type_name is None
        assert definition.aliases is None
        assert definition.get_token_type_name() == "MINIMAL"


class TestKeywordRegistryBasics:
    """Test basic registry functionality."""
    
    def test_registry_internal_structures(self):
        """Test internal data structures."""
        registry = LanguageKeywordRegistry()
        
        # Should have internal dictionaries
        assert hasattr(registry, '_keywords')
        assert hasattr(registry, '_aliases')
        
        # Should start empty or with predefined keywords
        assert isinstance(registry._keywords, dict)
        assert isinstance(registry._aliases, dict)
    
    def test_registry_is_independent(self):
        """Test that different registry instances are independent."""
        registry1 = LanguageKeywordRegistry()
        registry2 = LanguageKeywordRegistry()
        
        # Should be different objects
        assert registry1 is not registry2
        assert registry1._keywords is not registry2._keywords
        assert registry1._aliases is not registry2._aliases


class TestKeywordRegistryDataStructures:
    """Test keyword registry data structure integrity."""
    
    def test_keywords_dict_structure(self):
        """Test keywords dictionary structure."""
        registry = LanguageKeywordRegistry()
        
        # Should be a dictionary
        assert isinstance(registry._keywords, dict)
        
        # Values should be KeywordDefinition instances if any exist
        for keyword, definition in registry._keywords.items():
            assert isinstance(keyword, str)
            assert isinstance(definition, KeywordDefinition)
    
    def test_aliases_dict_structure(self):
        """Test aliases dictionary structure."""
        registry = LanguageKeywordRegistry()
        
        # Should be a dictionary
        assert isinstance(registry._aliases, dict)
        
        # Values should be strings if any exist
        for alias, canonical in registry._aliases.items():
            assert isinstance(alias, str)
            assert isinstance(canonical, str)


class TestKeywordCategoryUsage:
    """Test KeywordCategory usage in definitions."""
    
    def test_all_categories_usable(self):
        """Test that all categories can be used in definitions."""
        for category in KeywordCategory:
            definition = KeywordDefinition(
                keyword=f"test_{category.value}",
                category=category,
                parser_method=f"parse_{category.value}",
                description=f"Test {category.value} keyword",
                syntax_example=f"test_{category.value}"
            )
            
            assert definition.category == category
    
    def test_category_enum_completeness(self):
        """Test that category enum has expected values."""
        category_values = [c.value for c in KeywordCategory]
        
        # Should have these basic categories
        assert "type" in category_values
        assert "statement" in category_values
        assert "literal" in category_values