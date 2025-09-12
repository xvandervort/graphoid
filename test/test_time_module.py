"""Tests for the Time module functionality."""

import pytest
import time
import datetime as python_datetime
from glang.execution.pipeline import ExecutionSession


class TestTimeModule:
    """Test Time module operations."""
    
    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()
        # Import Time module with alias
        result = self.session.execute_statement('import "time" as Time')
        assert result.success, f"Failed to import Time module: {result}"
    
    def test_import_time_module(self):
        """Test that the time module can be imported."""
        # Already imported in setup_method
        result = self.session.execute_statement("Time")
        assert result.success
    
    def test_time_now(self):
        """Test Time.now() function."""
        before = time.time()
        result = self.session.execute_statement('current = Time.now()')
        after = time.time()
        
        assert result.success
        
        # Verify the current variable exists and has correct type
        result = self.session.execute_statement('current.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        # Verify timestamp is reasonable (within expected range)
        result = self.session.execute_statement('current')
        assert result.success
        # The result should be a TimeValue that displays as ISO format
        time_str = result.value.to_string()
        assert "T" in time_str  # Should be ISO format like "2025-09-12T02:33:03Z"
        assert time_str.endswith("Z")  # Should be UTC
    
    def test_time_today(self):
        """Test Time.today() function."""
        result = self.session.execute_statement('today = Time.today()')
        assert result.success
        
        # Verify type
        result = self.session.execute_statement('today.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        # Verify it's start of day (should end with 00:00:00Z)
        result = self.session.execute_statement('today.to_string()')
        assert result.success
        time_str = result.value.value
        assert time_str.endswith("T00:00:00Z")  # Should be midnight UTC
    
    def test_time_from_components_date_only(self):
        """Test Time.from_components() with date only."""
        result = self.session.execute_statement('birthday = Time.from_components(1990, 12, 25)')
        assert result.success
        
        # Verify type
        result = self.session.execute_statement('birthday.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        # Verify the date is correct
        result = self.session.execute_statement('birthday.to_string()')
        assert result.success
        time_str = result.value.value
        assert time_str.startswith("1990-12-25")
        assert time_str.endswith("T00:00:00Z")  # Should be midnight UTC
    
    def test_time_from_components_full_datetime(self):
        """Test Time.from_components() with full date and time."""
        result = self.session.execute_statement('meeting = Time.from_components(2025, 1, 15, 14, 30, 45)')
        assert result.success
        
        # Verify type
        result = self.session.execute_statement('meeting.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        # Verify the datetime is correct
        result = self.session.execute_statement('meeting.to_string()')
        assert result.success
        time_str = result.value.value
        assert time_str == "2025-01-15T14:30:45Z"
    
    def test_time_from_string_iso_format(self):
        """Test Time.from_string() with ISO format."""
        result = self.session.execute_statement('parsed = Time.from_string("2025-01-15T14:30:00")')
        assert result.success
        
        # Verify type
        result = self.session.execute_statement('parsed.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        # Verify the parsed time is correct
        result = self.session.execute_statement('parsed.to_string()')
        assert result.success
        time_str = result.value.value
        assert time_str == "2025-01-15T14:30:00Z"
    
    def test_time_from_string_with_z_suffix(self):
        """Test Time.from_string() with Z suffix."""
        result = self.session.execute_statement('parsed_z = Time.from_string("2025-01-15T14:30:00Z")')
        assert result.success
        
        # Verify the parsed time is correct
        result = self.session.execute_statement('parsed_z.to_string()')
        assert result.success
        time_str = result.value.value
        assert time_str == "2025-01-15T14:30:00Z"
    
    def test_time_method_calls(self):
        """Test method calls on time values."""
        # Create a time value
        result = self.session.execute_statement('test_time = Time.from_components(2025, 6, 15, 10, 30, 0)')
        assert result.success
        
        # Test get_type() method
        result = self.session.execute_statement('type_result = test_time.get_type()')
        assert result.success
        result = self.session.execute_statement('type_result')
        assert result.success
        assert result.value.value == "time"
        
        # Test to_string() method
        result = self.session.execute_statement('string_result = test_time.to_string()')
        assert result.success
        result = self.session.execute_statement('string_result')
        assert result.success
        assert result.value.value == "2025-06-15T10:30:00Z"
    
    def test_time_type_inference(self):
        """Test that time values are properly type-inferred."""
        # Type should be inferred when assigning from Time.now()
        result = self.session.execute_statement('inferred_time = Time.now()')
        assert result.success
        
        # Check that it was properly inferred as time type
        result = self.session.execute_statement('inferred_time.get_type()')
        assert result.success
        assert result.value.value == "time"
    
    def test_time_error_cases(self):
        """Test error cases for time functions."""
        # Invalid date components
        result = self.session.execute_statement('invalid_date = Time.from_components(2025, 13, 35)')  # Invalid month and day
        assert not result.success
        assert "Invalid date/time" in str(result.error)
        
        # Invalid string format
        result = self.session.execute_statement('invalid_string = Time.from_string("not-a-date")')
        assert not result.success
        assert "Invalid time format" in str(result.error)
        
        # Non-string argument to from_string
        result = self.session.execute_statement('invalid_arg = Time.from_string(12345)')
        assert not result.success
        assert "Time string must be a string" in str(result.error)
        
        # Non-number arguments to from_components
        result = self.session.execute_statement('invalid_components = Time.from_components("2025", 1, 1)')
        assert not result.success
        assert "Year, month, and day must be numbers" in str(result.error)
    
    def test_time_method_error_cases(self):
        """Test error cases for time methods."""
        # Create a time value
        self.session.execute_statement('test_time = Time.now()')
        
        # Test methods with wrong number of arguments
        result = self.session.execute_statement('test_time.get_type("extra_arg")')
        assert not result.success
        assert "get_type() takes no arguments" in str(result.error)
        
        result = self.session.execute_statement('test_time.to_string("extra_arg")')
        assert not result.success
        assert "to_string() takes no arguments" in str(result.error)
    
    def test_time_value_persistence(self):
        """Test that time values persist correctly across operations."""
        # Create a specific time
        result = self.session.execute_statement('fixed_time = Time.from_components(2025, 3, 15, 12, 0, 0)')
        assert result.success
        
        # Access it multiple times - should be consistent
        for i in range(3):
            result = self.session.execute_statement('fixed_time.to_string()')
            assert result.success
            assert result.value.value == "2025-03-15T12:00:00Z"
    
    def test_multiple_time_values(self):
        """Test working with multiple time values."""
        # Create multiple times
        result = self.session.execute_statement('time1 = Time.from_components(2025, 1, 1, 0, 0, 0)')
        assert result.success
        
        result = self.session.execute_statement('time2 = Time.from_components(2025, 12, 31, 23, 59, 59)')
        assert result.success
        
        # Verify both exist and have correct values
        result = self.session.execute_statement('time1.to_string()')
        assert result.success
        assert result.value.value == "2025-01-01T00:00:00Z"
        
        result = self.session.execute_statement('time2.to_string()')
        assert result.success
        assert result.value.value == "2025-12-31T23:59:59Z"
    
    def test_time_to_number_casting(self):
        """Test casting time values to numbers (timestamps)."""
        # Create a specific time
        result = self.session.execute_statement('test_time = Time.from_components(2025, 1, 1, 0, 0, 0)')
        assert result.success
        
        # Cast to number
        result = self.session.execute_statement('timestamp = test_time.to_num()')
        assert result.success
        
        # Check the timestamp value (should be Unix timestamp for 2025-01-01 00:00:00 UTC)
        result = self.session.execute_statement('timestamp')
        assert result.success
        assert result.value.get_type() == "num"
        # The exact timestamp should be 1735689600 for 2025-01-01T00:00:00Z
        assert result.value.value == 1735689600
    
    def test_number_to_time_casting(self):
        """Test casting numbers (timestamps) to time values."""
        # Start with a known timestamp
        result = self.session.execute_statement('timestamp = 1735689600')  # 2025-01-01T00:00:00Z
        assert result.success
        
        # Cast to time
        result = self.session.execute_statement('time_value = timestamp.to_time()')
        assert result.success
        
        # Check the time value
        result = self.session.execute_statement('time_value.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        result = self.session.execute_statement('time_value.to_string()')
        assert result.success
        assert result.value.value == "2025-01-01T00:00:00Z"
    
    def test_string_to_time_casting(self):
        """Test casting ISO format strings to time values."""
        # Start with an ISO format string
        result = self.session.execute_statement('time_str = "2025-01-15T14:30:00"')
        assert result.success
        
        # Cast to time
        result = self.session.execute_statement('time_value = time_str.to_time()')
        assert result.success
        
        # Check the time value
        result = self.session.execute_statement('time_value.get_type()')
        assert result.success
        assert result.value.value == "time"
        
        result = self.session.execute_statement('time_value.to_string()')
        assert result.success
        assert result.value.value == "2025-01-15T14:30:00Z"
    
    def test_time_casting_round_trips(self):
        """Test that time casting maintains consistency in round trips."""
        # Start with a specific time
        result = self.session.execute_statement('original = Time.from_components(2025, 6, 15, 10, 30, 45)')
        assert result.success
        
        # Time -> Number -> Time
        result = self.session.execute_statement('timestamp = original.to_num()')
        assert result.success
        
        result = self.session.execute_statement('from_number = timestamp.to_time()')
        assert result.success
        
        # Check consistency
        result = self.session.execute_statement('original.to_string()')
        assert result.success
        original_str = result.value.value
        
        result = self.session.execute_statement('from_number.to_string()')
        assert result.success
        assert result.value.value == original_str
        
        # Time -> String -> Time
        result = self.session.execute_statement('time_str = original.to_string()')
        assert result.success
        
        result = self.session.execute_statement('from_string = time_str.to_time()')
        assert result.success
        
        result = self.session.execute_statement('from_string.to_string()')
        assert result.success
        assert result.value.value == original_str
    
    def test_time_casting_error_cases(self):
        """Test error cases for time casting."""
        # Invalid string format for to_time()
        result = self.session.execute_statement('invalid_str = "not-a-time"')
        assert result.success
        
        result = self.session.execute_statement('invalid_str.to_time()')
        assert not result.success
        assert "Invalid time format" in str(result.error)
        
        # Test method calls with wrong number of arguments
        self.session.execute_statement('test_time = Time.now()')
        
        result = self.session.execute_statement('test_time.to_num("extra_arg")')
        assert not result.success
        assert "to_num() takes no arguments" in str(result.error)
        
        result = self.session.execute_statement('timestamp = 1735689600')
        result = self.session.execute_statement('timestamp.to_time("extra_arg")')
        assert not result.success
        assert "to_time() takes no arguments" in str(result.error)
        
        result = self.session.execute_statement('time_str = "2025-01-01T00:00:00"')
        result = self.session.execute_statement('time_str.to_time("extra_arg")')
        assert not result.success
        assert "to_time() takes no arguments" in str(result.error)