"""Tests for AST node classes."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.ast.nodes import *

class TestASTNodes:
    """Test AST node construction and visitor pattern."""
    
    def test_source_position(self):
        """Test SourcePosition class."""
        pos = SourcePosition(5, 10)
        assert pos.line == 5
        assert pos.column == 10
        assert str(pos) == "line 5, column 10"
    
    def test_variable_ref(self):
        """Test VariableRef node."""
        pos = SourcePosition(1, 5)
        node = VariableRef("myvar", pos)
        assert node.name == "myvar"
        assert node.position == pos
        
        # Test visitor acceptance
        class TestVisitor(ASTVisitor):
            def visit_variable_ref(self, node):
                return f"visited_{node.name}"
            def visit_string_literal(self, node): pass
            def visit_number_literal(self, node): pass
            def visit_boolean_literal(self, node): pass
            def visit_list_literal(self, node): pass
            def visit_data_node_literal(self, node): pass
            def visit_map_literal(self, node): pass
            def visit_index_access(self, node): pass
            def visit_slice_access(self, node): pass
            def visit_method_call_expression(self, node): pass
            def visit_print_expression(self, node): pass
            def visit_binary_operation(self, node): pass
            def visit_unary_operation(self, node): pass
            def visit_variable_declaration(self, node): pass
            def visit_method_call(self, node): pass
            def visit_assignment(self, node): pass
            def visit_index_assignment(self, node): pass
            def visit_slice_assignment(self, node): pass
            def visit_expression_statement(self, node): pass
            def visit_load_statement(self, node): pass
            def visit_print_statement(self, node): pass
            def visit_import_statement(self, node): pass
            def visit_module_declaration(self, node): pass
            def visit_alias_declaration(self, node): pass
            def visit_noop(self, node): pass
            # Control flow visitor methods
            def visit_if_statement(self, node): pass
            def visit_while_statement(self, node): pass
            def visit_for_in_statement(self, node): pass
            def visit_break_statement(self, node): pass
            def visit_continue_statement(self, node): pass
            def visit_block(self, node): pass
            # Function-related visitor methods
            def visit_function_declaration(self, node): pass
            def visit_return_statement(self, node): pass
            def visit_function_call(self, node): pass
            def visit_lambda_expression(self, node): pass
            
        visitor = TestVisitor()
        result = node.accept(visitor)
        assert result == "visited_myvar"
    
    def test_literals(self):
        """Test literal nodes."""
        # String literal
        str_node = StringLiteral("hello")
        assert str_node.value == "hello"
        
        # Number literal (int)
        num_node = NumberLiteral(42)
        assert num_node.value == 42
        assert isinstance(num_node.value, int)
        
        # Number literal (float)  
        float_node = NumberLiteral(3.14)
        assert float_node.value == 3.14
        assert isinstance(float_node.value, float)
        
        # Boolean literal
        bool_node = BooleanLiteral(True)
        assert bool_node.value == True
    
    def test_list_literal(self):
        """Test ListLiteral node."""
        elements = [
            StringLiteral("apple"),
            StringLiteral("banana"),
            NumberLiteral(42)
        ]
        list_node = ListLiteral(elements)
        assert len(list_node.elements) == 3
        assert isinstance(list_node.elements[0], StringLiteral)
        assert list_node.elements[0].value == "apple"
        assert isinstance(list_node.elements[2], NumberLiteral)
        assert list_node.elements[2].value == 42
    
    def test_index_access(self):
        """Test IndexAccess node."""
        target = VariableRef("arr")
        indices = [NumberLiteral(0)]
        index_node = IndexAccess(target, indices)
        
        assert isinstance(index_node.target, VariableRef)
        assert index_node.target.name == "arr"
        assert len(index_node.indices) == 1
        assert index_node.indices[0].value == 0
    
    def test_slice_access(self):
        """Test SliceAccess node."""
        target = VariableRef("arr")
        start = NumberLiteral(1)
        stop = NumberLiteral(3)
        step = NumberLiteral(2)
        
        slice_node = SliceAccess(target, start, stop, step)
        assert isinstance(slice_node.target, VariableRef)
        assert slice_node.start.value == 1
        assert slice_node.stop.value == 3  
        assert slice_node.step.value == 2
        
        # Test with optional parts
        slice_node2 = SliceAccess(target, None, stop, None)
        assert slice_node2.start is None
        assert slice_node2.step is None
        assert slice_node2.stop.value == 3
    
    def test_method_call_expression(self):
        """Test MethodCallExpression node."""
        target = VariableRef("obj")
        arguments = [StringLiteral("arg1"), NumberLiteral(42)]
        method_node = MethodCallExpression(target, "append", arguments)
        
        assert isinstance(method_node.target, VariableRef)
        assert method_node.method_name == "append"
        assert len(method_node.arguments) == 2
        assert method_node.arguments[0].value == "arg1"
        assert method_node.arguments[1].value == 42
    
    def test_variable_declaration(self):
        """Test VariableDeclaration node."""
        initializer = StringLiteral("hello")
        decl = VariableDeclaration("string", "name", initializer)
        
        assert decl.var_type == "string"
        assert decl.name == "name"
        assert isinstance(decl.initializer, StringLiteral)
        assert decl.type_constraint is None
        
        # Test with type constraint
        list_init = ListLiteral([NumberLiteral(1), NumberLiteral(2)])
        decl2 = VariableDeclaration("list", "numbers", list_init, "num")
        assert decl2.type_constraint == "num"
    
    def test_method_call_statement(self):
        """Test MethodCall statement node."""
        target = VariableRef("obj")  
        arguments = [StringLiteral("value")]
        method_stmt = MethodCall(target, "append", arguments)
        
        assert isinstance(method_stmt.target, VariableRef)
        assert method_stmt.method_name == "append"
        assert len(method_stmt.arguments) == 1
    
    def test_assignments(self):
        """Test assignment statement nodes."""
        # Index assignment
        target = IndexAccess(VariableRef("arr"), [NumberLiteral(0)])
        value = StringLiteral("new_value")
        index_assign = IndexAssignment(target, value)
        
        assert isinstance(index_assign.target, IndexAccess)
        assert isinstance(index_assign.value, StringLiteral)
        
        # Slice assignment
        slice_target = SliceAccess(VariableRef("arr"), NumberLiteral(1), NumberLiteral(3), None)
        list_value = ListLiteral([StringLiteral("a"), StringLiteral("b")])
        slice_assign = SliceAssignment(slice_target, list_value)
        
        assert isinstance(slice_assign.target, SliceAccess)
        assert isinstance(slice_assign.value, ListLiteral)
    
    def test_expression_statement(self):
        """Test ExpressionStatement node."""
        expr = VariableRef("myvar")
        stmt = ExpressionStatement(expr)
        
        assert isinstance(stmt.expression, VariableRef)
        assert stmt.expression.name == "myvar"
    

class TestBaseVisitor:
    """Test the BaseASTVisitor implementation."""
    
    def test_base_visitor(self):
        """Test BaseASTVisitor default implementations."""
        visitor = BaseASTVisitor()
        
        # Test simple nodes
        var_ref = VariableRef("test")
        result = visitor.visit_variable_ref(var_ref)
        assert result == var_ref
        
        str_lit = StringLiteral("hello")
        result = visitor.visit_string_literal(str_lit)
        assert result == str_lit
        
        # Test complex nodes (should traverse children)
        elements = [StringLiteral("a"), NumberLiteral(1)]
        list_lit = ListLiteral(elements)
        
        class CountingVisitor(BaseASTVisitor):
            def __init__(self):
                self.visit_count = 0
            
            def visit_string_literal(self, node):
                self.visit_count += 1
                return super().visit_string_literal(node)
            
            def visit_number_literal(self, node):
                self.visit_count += 1
                return super().visit_number_literal(node)
        
        counting_visitor = CountingVisitor()
        counting_visitor.visit_list_literal(list_lit)
        assert counting_visitor.visit_count == 2  # Should visit both elements
    
    def test_visitor_traversal(self):
        """Test that visitor properly traverses nested structures."""
        # Create: obj.method(arg[0])
        index_arg = IndexAccess(VariableRef("arg"), [NumberLiteral(0)])
        method_expr = MethodCallExpression(VariableRef("obj"), "method", [index_arg])
        
        class CollectingVisitor(BaseASTVisitor):
            def __init__(self):
                self.visited_nodes = []
            
            def visit_variable_ref(self, node):
                self.visited_nodes.append(f"var:{node.name}")
                return super().visit_variable_ref(node)
            
            def visit_number_literal(self, node):
                self.visited_nodes.append(f"num:{node.value}")
                return super().visit_number_literal(node)
        
        visitor = CollectingVisitor()
        visitor.visit_method_call_expression(method_expr)
        
        # Should visit: obj, arg, 0
        assert "var:obj" in visitor.visited_nodes
        assert "var:arg" in visitor.visited_nodes  
        assert "num:0" in visitor.visited_nodes
        assert len(visitor.visited_nodes) == 3


if __name__ == '__main__':
    pytest.main([__file__])