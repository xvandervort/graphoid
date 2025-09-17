"""
Configuration Context Management for Glang

Manages a stack of configuration settings that control behavior at different scopes:
- System defaults (base)
- File-level configuration
- Function-level configuration
- Block-level configuration (configure blocks)

Settings cascade from outer to inner scopes, with inner settings overriding outer ones.
"""

from typing import Dict, Any, List, Optional, Union
from dataclasses import dataclass, field


@dataclass
class ConfigurationLayer:
    """A single layer of configuration settings."""
    settings: Dict[str, Any] = field(default_factory=dict)
    scope_name: str = "default"

    def get(self, key: str, default: Any = None) -> Any:
        """Get a setting value with optional default."""
        return self.settings.get(key, default)

    def set(self, key: str, value: Any) -> None:
        """Set a configuration value."""
        self.settings[key] = value

    def has(self, key: str) -> bool:
        """Check if a setting exists."""
        return key in self.settings


class ConfigurationContext:
    """
    Manages a stack of configuration layers with inheritance.

    Configuration settings control runtime behavior like:
    - skip_none: Whether to skip none values in operations (default: False)
    - decimal_places: Number of decimal places for numeric operations (default: None)
    - strict_types: Whether to enforce strict type checking (default: False)
    - allow_implicit_conversion: Whether to allow implicit type conversions (default: True)
    """

    # Default system-wide settings
    SYSTEM_DEFAULTS = {
        'skip_none': False,           # Error on none values by default
        'decimal_places': None,       # No decimal rounding by default
        'strict_types': False,        # Allow flexible typing by default
        'allow_implicit_conversion': True,  # Allow implicit conversions
        'auto_flatten': False,        # Don't auto-flatten nested lists
        'case_sensitive': True,       # String operations are case-sensitive
        'zero_division': 'error',     # Error on division by zero (vs 'infinity' or 'none')
    }

    # Valid configuration keys and their types
    VALID_SETTINGS = {
        'skip_none': bool,
        'skip_nil': bool,  # Alias for skip_none
        'decimal_places': (int, type(None)),
        'strict_types': bool,
        'allow_implicit_conversion': bool,
        'auto_flatten': bool,
        'case_sensitive': bool,
        'zero_division': str,
    }

    def __init__(self):
        """Initialize with system defaults as the base layer."""
        base_layer = ConfigurationLayer(
            settings=self.SYSTEM_DEFAULTS.copy(),
            scope_name="system"
        )
        self._stack: List[ConfigurationLayer] = [base_layer]

    def push_configuration(self, config_dict: Dict[str, Any], scope_name: str = "block") -> None:
        """
        Push a new configuration layer onto the stack.

        Args:
            config_dict: Dictionary of configuration settings
            scope_name: Name of the scope (e.g., "block", "function", "file")
        """
        # Validate and normalize settings
        normalized = self._normalize_settings(config_dict)

        # Create new layer with normalized settings
        layer = ConfigurationLayer(settings=normalized, scope_name=scope_name)
        self._stack.append(layer)

    def pop_configuration(self) -> Optional[ConfigurationLayer]:
        """
        Pop the top configuration layer from the stack.

        Returns:
            The popped layer, or None if only system defaults remain
        """
        if len(self._stack) > 1:  # Keep system defaults
            return self._stack.pop()
        return None

    def get_setting(self, key: str, default: Any = None) -> Any:
        """
        Get a configuration setting, checking from top to bottom of stack.

        Args:
            key: Setting key to look up
            default: Default value if not found

        Returns:
            The setting value from the highest priority layer that has it
        """
        # Normalize key (handle aliases)
        key = self._normalize_key(key)

        # Search from top (highest priority) to bottom
        for layer in reversed(self._stack):
            if layer.has(key):
                return layer.get(key)

        return default

    def is_enabled(self, setting: str) -> bool:
        """
        Check if a boolean setting is enabled.

        Args:
            setting: Name of the boolean setting

        Returns:
            True if the setting is enabled, False otherwise
        """
        value = self.get_setting(setting, False)
        return bool(value)

    def get_decimal_places(self) -> Optional[int]:
        """
        Get the current decimal places setting.

        Returns:
            Number of decimal places, or None for no rounding
        """
        return self.get_setting('decimal_places')

    def should_skip_none(self) -> bool:
        """
        Check if none values should be skipped in operations.

        Returns:
            True if none values should be skipped, False if they should cause errors
        """
        # Check both skip_none and skip_nil (alias)
        return self.is_enabled('skip_none') or self.is_enabled('skip_nil')

    def get_zero_division_behavior(self) -> str:
        """
        Get the behavior for division by zero.

        Returns:
            'error', 'infinity', or 'none'
        """
        return self.get_setting('zero_division', 'error')

    def _normalize_key(self, key: str) -> str:
        """
        Normalize configuration keys (handle aliases).

        Args:
            key: The key to normalize

        Returns:
            The normalized key
        """
        # Handle aliases
        if key == 'skip_nil':
            return 'skip_none'
        return key

    def _normalize_settings(self, config_dict: Dict[str, Any]) -> Dict[str, Any]:
        """
        Validate and normalize configuration settings.

        Args:
            config_dict: Raw configuration dictionary

        Returns:
            Normalized configuration dictionary

        Raises:
            ValueError: If invalid settings are provided
        """
        normalized = {}

        for key, value in config_dict.items():
            # Normalize the key
            norm_key = self._normalize_key(key)

            # Check if it's a valid setting
            if norm_key not in self.VALID_SETTINGS:
                # For now, silently ignore unknown settings
                # In the future, we might want to warn or error
                continue

            # Type check the value
            expected_type = self.VALID_SETTINGS[norm_key]
            if isinstance(expected_type, tuple):
                # Multiple valid types
                if not any(isinstance(value, t) if t is not type(None) else value is None
                          for t in expected_type):
                    raise ValueError(
                        f"Configuration '{norm_key}' expects type {expected_type}, "
                        f"got {type(value).__name__}"
                    )
            else:
                # Single valid type
                if not isinstance(value, expected_type):
                    raise ValueError(
                        f"Configuration '{norm_key}' expects type {expected_type.__name__}, "
                        f"got {type(value).__name__}"
                    )

            # Additional validation for specific settings
            if norm_key == 'decimal_places' and value is not None:
                if value < 0:
                    raise ValueError(f"decimal_places must be non-negative, got {value}")
                if value > 100:
                    raise ValueError(f"decimal_places must be <= 100, got {value}")

            if norm_key == 'zero_division':
                if value not in ['error', 'infinity', 'none']:
                    raise ValueError(
                        f"zero_division must be 'error', 'infinity', or 'none', got '{value}'"
                    )

            normalized[norm_key] = value

        return normalized

    def get_current_configuration(self) -> Dict[str, Any]:
        """
        Get the effective configuration (merged from all layers).

        Returns:
            Dictionary of all current settings
        """
        result = {}

        # Build from bottom to top (system to current)
        for layer in self._stack:
            result.update(layer.settings)

        return result

    def __repr__(self) -> str:
        """String representation for debugging."""
        layers = [f"{layer.scope_name}: {layer.settings}" for layer in self._stack]
        return f"ConfigurationContext({' -> '.join(layers)})"