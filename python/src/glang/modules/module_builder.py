"""
Module Builder utility to reduce boilerplate in module creation.

This utility extracts the common pattern used across all Glang modules
to create and populate module namespaces with functions.
"""

from typing import Dict, Callable, Any, Optional
from .module_manager import ModuleNamespace
from ..execution.function_value import BuiltinFunctionValue


class ModuleBuilder:
    """Builder class for creating Glang modules with minimal boilerplate."""

    def __init__(self, module_name: str):
        """Initialize a module builder.

        Args:
            module_name: The name of the module being built
        """
        self.module_name = module_name
        self.namespace = ModuleNamespace(module_name)
        self.functions: Dict[str, Callable] = {}
        self.variables: Dict[str, Any] = {}

    def add_function(self, name: str, func: Callable) -> 'ModuleBuilder':
        """Add a function to the module.

        Args:
            name: The name to expose the function as in Glang
            func: The Python callable implementing the function

        Returns:
            self for method chaining
        """
        self.functions[name] = func
        return self

    def add_functions(self, functions: Dict[str, Callable]) -> 'ModuleBuilder':
        """Add multiple functions to the module.

        Args:
            functions: Dictionary mapping function names to callables

        Returns:
            self for method chaining
        """
        self.functions.update(functions)
        return self

    def add_variable(self, name: str, value: Any) -> 'ModuleBuilder':
        """Add a variable/constant to the module.

        Args:
            name: The name of the variable
            value: The value to assign

        Returns:
            self for method chaining
        """
        self.variables[name] = value
        return self

    def add_variables(self, variables: Dict[str, Any]) -> 'ModuleBuilder':
        """Add multiple variables/constants to the module.

        Args:
            variables: Dictionary mapping variable names to values

        Returns:
            self for method chaining
        """
        self.variables.update(variables)
        return self

    def build(self) -> ModuleNamespace:
        """Build and return the completed module namespace.

        Returns:
            The fully configured ModuleNamespace
        """
        # Add all functions as BuiltinFunctionValues
        for name, func in self.functions.items():
            self.namespace.set_symbol(name, BuiltinFunctionValue(name, func))

        # Add all variables directly
        for name, value in self.variables.items():
            self.namespace.set_symbol(name, value)

        return self.namespace


def create_module(module_name: str,
                 functions: Optional[Dict[str, Callable]] = None,
                 variables: Optional[Dict[str, Any]] = None,
                 module_instance: Optional[Any] = None) -> ModuleNamespace:
    """Convenience function to create a module in one call.

    Args:
        module_name: Name of the module
        functions: Dictionary of function name to callable mappings
        variables: Dictionary of variable name to value mappings
        module_instance: Optional module class instance for method extraction

    Returns:
        Configured ModuleNamespace ready for use
    """
    builder = ModuleBuilder(module_name)

    if functions:
        builder.add_functions(functions)

    if variables:
        builder.add_variables(variables)

    # If a module instance is provided, extract its methods
    if module_instance:
        # This is useful for modules like RandomModule that have instance state
        instance_methods = {}
        for attr_name in dir(module_instance):
            if not attr_name.startswith('_'):  # Skip private methods
                attr = getattr(module_instance, attr_name)
                if callable(attr):
                    instance_methods[attr_name] = attr
        builder.add_functions(instance_methods)

    return builder.build()