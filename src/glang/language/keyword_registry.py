"""
Centralized Keyword Registry for Glang Language

This module defines ALL language keywords, their parsing behavior, and metadata
in a single, maintainable location. Adding new keywords requires only updating
this registry - no changes to tokenizer or parser files.
"""

from typing import Dict, Callable, Optional, List
from enum import Enum
from dataclasses import dataclass


class KeywordCategory(Enum):
    """Categories of keywords for organization."""
    TYPE = "type"              # string, num, bool, list
    STATEMENT = "statement"    # import, module, alias, load
    LITERAL = "literal"        # true, false


@dataclass
class KeywordDefinition:
    """Complete definition of a language keyword."""
    keyword: str
    category: KeywordCategory
    parser_method: str
    description: str
    syntax_example: str
    token_type_name: Optional[str] = None  # If needs special token type
    aliases: Optional[List[str]] = None    # Alternative forms (like /import)
    
    def get_token_type_name(self) -> str:
        """Get the token type name for this keyword."""
        if self.token_type_name:
            return self.token_type_name
        # Auto-generate from keyword: "import" -> "IMPORT"
        return self.keyword.upper()


class LanguageKeywordRegistry:
    """
    Central registry for all language keywords.
    
    This is the SINGLE SOURCE OF TRUTH for all language keywords.
    Adding new language features requires only updating this registry.
    """
    
    def __init__(self):
        self._keywords: Dict[str, KeywordDefinition] = {}
        self._aliases: Dict[str, str] = {}  # alias -> canonical_keyword
        self._parser_methods: Dict[str, str] = {}  # keyword -> parser_method
        self._initialize_keywords()
    
    def _initialize_keywords(self):
        """Initialize all language keywords."""
        
        # Type keywords
        self._register_keyword(KeywordDefinition(
            keyword="string",
            category=KeywordCategory.TYPE,
            parser_method="parse_type_keyword",
            description="String type declaration",
            syntax_example="string name = \"value\""
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="num",
            category=KeywordCategory.TYPE,
            parser_method="parse_type_keyword",
            description="Number type declaration", 
            syntax_example="num count = 42"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="bool",
            category=KeywordCategory.TYPE,
            parser_method="parse_type_keyword",
            description="Boolean type declaration",
            syntax_example="bool active = true"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="list",
            category=KeywordCategory.TYPE,
            parser_method="parse_type_keyword",
            description="List type declaration",
            syntax_example="list<string> items = [\"a\", \"b\"]"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="data",
            category=KeywordCategory.TYPE,
            parser_method="parse_type_keyword",
            description="Data node type declaration",
            syntax_example="data node = { \"key\": \"value\" }"
        ))
        
        # Statement keywords
        self._register_keyword(KeywordDefinition(
            keyword="import",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_import_statement_without_slash",
            description="Import a module file",
            syntax_example="import \"math.gr\" as math",
            aliases=["/import"]  # Backward compatibility
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="module",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_module_declaration",
            description="Declare module name",
            syntax_example="module mathematics"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="alias",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_alias_declaration",
            description="Declare module alias",
            syntax_example="alias math"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="load",
            category=KeywordCategory.STATEMENT,
            parser_method="parse_load_statement",
            description="Load and execute a file",
            syntax_example="load \"config.gr\""
        ))
        
        # Boolean literal keywords
        self._register_keyword(KeywordDefinition(
            keyword="true",
            category=KeywordCategory.LITERAL,
            parser_method="parse_boolean_literal",
            description="Boolean true value",
            syntax_example="bool flag = true"
        ))
        
        self._register_keyword(KeywordDefinition(
            keyword="false",
            category=KeywordCategory.LITERAL,
            parser_method="parse_boolean_literal",
            description="Boolean false value",
            syntax_example="bool flag = false"
        ))
    
    def _register_keyword(self, definition: KeywordDefinition):
        """Register a keyword definition."""
        self._keywords[definition.keyword] = definition
        self._parser_methods[definition.keyword] = definition.parser_method
        
        # Register aliases
        if definition.aliases:
            for alias in definition.aliases:
                self._aliases[alias] = definition.keyword
    
    def is_keyword(self, text: str) -> bool:
        """Check if text is a registered keyword or alias."""
        return text.lower() in self._keywords or text in self._aliases
    
    def get_keyword(self, text: str) -> Optional[KeywordDefinition]:
        """Get keyword definition, resolving aliases."""
        text_lower = text.lower()
        
        # Check direct keyword
        if text_lower in self._keywords:
            return self._keywords[text_lower]
        
        # Check aliases
        if text in self._aliases:
            canonical = self._aliases[text]
            return self._keywords[canonical]
        
        return None
    
    def get_parser_method(self, text: str) -> Optional[str]:
        """Get parser method name for keyword."""
        definition = self.get_keyword(text)
        return definition.parser_method if definition else None
    
    def get_token_type_name(self, text: str) -> Optional[str]:
        """Get token type name for keyword."""
        definition = self.get_keyword(text)
        return definition.get_token_type_name() if definition else None
    
    def get_all_keywords(self) -> Dict[str, KeywordDefinition]:
        """Get all registered keywords."""
        return self._keywords.copy()
    
    def get_keywords_by_category(self, category: KeywordCategory) -> Dict[str, KeywordDefinition]:
        """Get keywords by category."""
        return {
            k: v for k, v in self._keywords.items() 
            if v.category == category
        }
    
    def get_all_recognized_forms(self) -> List[str]:
        """Get all forms that should be recognized (keywords + aliases)."""
        forms = list(self._keywords.keys())
        forms.extend(self._aliases.keys())
        return forms
    
    def add_keyword(self, definition: KeywordDefinition):
        """
        Add a new keyword to the registry.
        
        This is the ONLY method needed to extend the language with new keywords!
        """
        self._register_keyword(definition)
        # Refresh tokenizer's TokenType enum to include new keyword
        self._refresh_tokenizer_token_types()
    
    def _refresh_tokenizer_token_types(self):
        """Refresh the tokenizer's TokenType enum when keywords change."""
        try:
            # Import here to avoid circular imports
            import sys
            if 'glang.lexer.tokenizer' in sys.modules:
                tokenizer_module = sys.modules['glang.lexer.tokenizer']
                if hasattr(tokenizer_module, '_refresh_token_type'):
                    tokenizer_module._refresh_token_type()
        except Exception:
            # If refresh fails, it's not critical - the tokenizer will work
            # with existing keywords, just new ones won't be recognized as
            # keyword tokens (they'll be identifiers that get converted later)
            pass
    
    def remove_keyword(self, keyword: str):
        """Remove a keyword from the registry."""
        if keyword in self._keywords:
            definition = self._keywords[keyword]
            del self._keywords[keyword]
            del self._parser_methods[keyword]
            
            # Remove aliases
            if definition.aliases:
                for alias in definition.aliases:
                    if alias in self._aliases:
                        del self._aliases[alias]
    
    def generate_documentation(self) -> str:
        """Generate documentation for all keywords."""
        doc = "# Glang Language Keywords\n\n"
        
        for category in KeywordCategory:
            keywords = self.get_keywords_by_category(category)
            if not keywords:
                continue
                
            doc += f"## {category.value.title()} Keywords\n\n"
            
            for keyword, definition in sorted(keywords.items()):
                doc += f"### `{keyword}`\n"
                doc += f"**Description:** {definition.description}\n\n"
                doc += f"**Example:** `{definition.syntax_example}`\n\n"
                if definition.aliases:
                    doc += f"**Aliases:** {', '.join(f'`{a}`' for a in definition.aliases)}\n\n"
                doc += "---\n\n"
        
        return doc


# Global registry instance - SINGLE SOURCE OF TRUTH
KEYWORD_REGISTRY = LanguageKeywordRegistry()


# Convenience functions for easy access
def is_keyword(text: str) -> bool:
    """Check if text is a language keyword."""
    return KEYWORD_REGISTRY.is_keyword(text)


def get_keyword_definition(text: str) -> Optional[KeywordDefinition]:
    """Get keyword definition."""
    return KEYWORD_REGISTRY.get_keyword(text)


def get_parser_method_name(text: str) -> Optional[str]:
    """Get parser method name for keyword."""
    return KEYWORD_REGISTRY.get_parser_method(text)


def get_token_type_name(text: str) -> Optional[str]:
    """Get token type name for keyword."""
    return KEYWORD_REGISTRY.get_token_type_name(text)


def add_language_keyword(definition: KeywordDefinition):
    """Add a new keyword to the language."""
    KEYWORD_REGISTRY.add_keyword(definition)


# Example of how easy it is to extend the language:
# add_language_keyword(KeywordDefinition(
#     keyword="function",
#     category=KeywordCategory.STATEMENT,
#     parser_method="parse_function_declaration",
#     description="Declare a function",
#     syntax_example="function add(a: num, b: num) -> num { return a + b }"
# ))