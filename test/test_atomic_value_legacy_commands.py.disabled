"""
Tests to ensure AtomicValue works properly with legacy commands and doesn't crash the REPL.
"""

import pytest
from glang.repl.repl import REPL


class TestAtomicValueLegacyCommandsNoCrash:
    """Test that AtomicValue doesn't crash legacy commands."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
        # Create an atomic value for testing
        self.repl._process_input('string test_var = "hello"')
    
    def test_show_command_no_crash(self):
        """Test that /show doesn't crash with AtomicValue."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/show test_var')
            output = mock_stdout.getvalue()
            
            assert "Variable 'test_var' is an atomic string value" in output
            assert "Use 'test_var' (without /show) to display atomic values" in output
    
    def test_traverse_command_no_crash(self):
        """Test that /traverse doesn't crash with AtomicValue."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/traverse test_var')
            output = mock_stdout.getvalue()
            
            assert "Cannot traverse atomic value" in output
            assert "Traversal is only available for graphs" in output
    
    def test_info_command_no_crash(self):
        """Test that /info doesn't crash with AtomicValue."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/info test_var')
            output = mock_stdout.getvalue()
            
            assert "Variable: test_var" in output
            assert "Type: atomic_string" in output
            assert "Size: 1 nodes" in output
            assert "Edges: 0" in output
            assert "Value: 'hello'" in output
    
    def test_delete_command_no_crash(self):
        """Test that /delete doesn't crash with AtomicValue."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/delete test_var')
            output = mock_stdout.getvalue()
            
            assert "Deleted" in output
    
    def test_multiple_atomic_value_types(self):
        """Test all atomic value types with legacy commands."""
        test_cases = [
            ('string', 'text_val', '"test"'),
            ('num', 'num_val', '42'),
            ('bool', 'bool_val', 'true')
        ]
        
        for atomic_type, var_name, value in test_cases:
            # Create the variable
            self.repl._process_input(f'{atomic_type} {var_name} = {value}')
            
            # Test show command
            import io
            from unittest.mock import patch
            
            with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
                self.repl._process_input(f'/show {var_name}')
                output = mock_stdout.getvalue()
                assert f"atomic {atomic_type} value" in output
            
            # Test info command  
            with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
                self.repl._process_input(f'/info {var_name}')
                output = mock_stdout.getvalue()
                assert f"Type: atomic_{atomic_type}" in output
    
    def test_graphs_command_still_works(self):
        """Test that /graphs command still lists variables properly."""
        # Create both atomic and graph variables
        self.repl._process_input('num age = 25')
        self.repl._process_input('list items = [1, 2, 3]')
        
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/graphs')
            output = mock_stdout.getvalue()
            
            # Should list all variables (both atomic and graphs)
            assert "test_var" in output
            assert "age" in output  
            assert "items" in output
    
    def test_namespace_command_with_atomic_values(self):
        """Test that /namespace command works with mixed variable types."""
        self.repl._process_input('num count = 10')
        self.repl._process_input('list data = ["a", "b", "c"]')
        
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/namespace')
            output = mock_stdout.getvalue()
            
            # Should show the variable namespace graph
            assert "Variable Graph" in output or "namespace" in output.lower()
    
    def test_stats_command_with_atomic_values(self):
        """Test that /stats command works with mixed variable types.""" 
        self.repl._process_input('bool flag = false')
        
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/stats')
            output = mock_stdout.getvalue()
            
            # Should show stats about variables
            assert "Variable" in output
    
    def test_no_current_graph_with_atomic_values(self):
        """Test that atomic values don't become current graph."""
        import io
        from unittest.mock import patch
        
        # Atomic values shouldn't set current graph
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/append test')
            output = mock_stdout.getvalue()
            
            assert "No current graph" in output
    
    def test_legacy_create_after_atomic(self):
        """Test that legacy create still works after creating atomic values."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/create items [x, y, z]')
            output = mock_stdout.getvalue()
            
            assert "Created linear graph 'items'" in output
        
        # Now graph operations should work on the new current graph
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/append w')
            output = mock_stdout.getvalue()
            
            assert "Appended" in output or "Added" in output


class TestAtomicValueErrorHandling:
    """Test proper error handling for AtomicValue edge cases."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_show_nonexistent_variable(self):
        """Test /show with nonexistent variable."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/show nonexistent')
            output = mock_stdout.getvalue()
            
            assert "not found" in output.lower() or "no graphs available" in output.lower()
    
    def test_info_nonexistent_variable(self):
        """Test /info with nonexistent variable."""
        import io
        from unittest.mock import patch
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/info nonexistent')
            output = mock_stdout.getvalue()
            
            assert "not found" in output.lower()
    
    def test_operations_on_mixed_types(self):
        """Test that operations work correctly on mixed variable types."""
        self.repl._process_input('string name = "Alice"')
        self.repl._process_input('list hobbies = ["reading", "coding"]')
        
        import io
        from unittest.mock import patch
        
        # Show should work on both
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/show name')
            output = mock_stdout.getvalue()
            assert "atomic string value" in output
        
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/show hobbies')
            output = mock_stdout.getvalue()
            assert "Graph 'hobbies'" in output
        
        # Info should work on both
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/info name')
            output = mock_stdout.getvalue()
            assert "atomic_string" in output
            
        with patch('sys.stdout', new_callable=io.StringIO) as mock_stdout:
            self.repl._process_input('/info hobbies')
            output = mock_stdout.getvalue()
            assert "linear" in output.lower()