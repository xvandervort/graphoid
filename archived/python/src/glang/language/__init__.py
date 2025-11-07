"""Glang language definition and keyword management."""

from .keyword_registry import (
    LanguageKeywordRegistry,
    KeywordDefinition, 
    KeywordCategory,
    KEYWORD_REGISTRY,
    is_keyword,
    get_keyword_definition,
    get_parser_method_name,
    get_token_type_name,
    add_language_keyword
)

__all__ = [
    'LanguageKeywordRegistry',
    'KeywordDefinition',
    'KeywordCategory', 
    'KEYWORD_REGISTRY',
    'is_keyword',
    'get_keyword_definition',
    'get_parser_method_name',
    'get_token_type_name',
    'add_language_keyword'
]