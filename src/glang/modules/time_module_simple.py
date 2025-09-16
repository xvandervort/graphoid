"""
Simple Time module for Glang - Single Type Implementation

A single Time type that represents any point in time internally as UTC timestamp,
with natural methods for different representations and calendar-aware arithmetic.
"""

import datetime as python_datetime
import time
import calendar
from typing import Optional, Any, Dict, List, Union
from decimal import Decimal

from ..execution.values import (
    GlangValue, StringValue, BooleanValue, NumberValue, 
    ListValue, DataValue, HashValue, NoneValue, TimeValue
)
from ..execution.errors import RuntimeError
from ..execution.glang_number import PrecisionGlangNumber, create_glang_number, GlangNumber
from ..ast.nodes import SourcePosition
from .module_manager import ModuleNamespace



class TimeModule:
    """Built-in Time module providing a single Time type with natural methods."""
    
    @staticmethod
    def now(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get current time as Time value."""
        try:
            current_timestamp = time.time()
            return TimeValue(current_timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get current time: {str(e)}", position)
    
    @staticmethod
    def today(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get start of today (00:00:00 UTC)."""
        try:
            today = python_datetime.date.today()
            today_datetime = python_datetime.datetime.combine(today, python_datetime.time.min, python_datetime.timezone.utc)
            timestamp = today_datetime.timestamp()
            return TimeValue(timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get today: {str(e)}", position)
    
    @staticmethod
    def from_components(year: GlangValue, month: GlangValue, day: GlangValue,
                       hour: Optional[GlangValue] = None, minute: Optional[GlangValue] = None, 
                       second: Optional[GlangValue] = None,
                       position: Optional[SourcePosition] = None) -> GlangValue:
        """Create Time from date/time components."""
        try:
            # Validate and extract components
            if not all(isinstance(val, NumberValue) for val in [year, month, day]):
                raise RuntimeError("Year, month, and day must be numbers", position)
            
            year_val = int(year.to_python())
            month_val = int(month.to_python())
            day_val = int(day.to_python())
            hour_val = int(hour.to_python()) if hour and isinstance(hour, NumberValue) else 0
            minute_val = int(minute.to_python()) if minute and isinstance(minute, NumberValue) else 0
            second_val = int(second.to_python()) if second and isinstance(second, NumberValue) else 0
            
            # Create datetime in UTC
            dt = python_datetime.datetime(year_val, month_val, day_val, hour_val, minute_val, second_val, tzinfo=python_datetime.timezone.utc)
            timestamp = dt.timestamp()
            return TimeValue(timestamp, position)
            
        except ValueError as e:
            raise RuntimeError(f"Invalid date/time: {str(e)}", position)
        except Exception as e:
            raise RuntimeError(f"Failed to create time: {str(e)}", position)
    
    @staticmethod
    def from_string(time_str: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Parse Time from ISO string format."""
        try:
            if not isinstance(time_str, StringValue):
                raise RuntimeError("Time string must be a string", position)
            
            time_str_val = time_str.to_python()
            
            # Support basic ISO format: "2025-01-15T14:30:00"
            try:
                dt = python_datetime.datetime.fromisoformat(time_str_val.replace('Z', '+00:00'))
                if dt.tzinfo is None:
                    dt = dt.replace(tzinfo=python_datetime.timezone.utc)
                timestamp = dt.timestamp()
                return TimeValue(timestamp, position)
            except ValueError:
                raise RuntimeError(f"Invalid time format: {time_str_val}. Expected ISO format like '2025-01-15T14:30:00'", position)
                
        except Exception as e:
            raise RuntimeError(f"Failed to parse time string: {str(e)}", position)


def create_time_module_namespace():
    """Create the namespace for the built-in Time module."""
    from .module_builder import create_module

    return create_module(
        "time",
        functions={
            'now': TimeModule.now,
            'today': TimeModule.today,
            'from_components': TimeModule.from_components,
            'from_string': TimeModule.from_string,
        }
    )