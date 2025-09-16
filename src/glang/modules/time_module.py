"""
Built-in Time module for Glang

A single Time type that can represent any point in time - internally a UTC timestamp,
with natural methods for different representations and calendar-aware arithmetic.

Key principles:
- Single type simplicity: One Time, many representations  
- Natural methods: .as_date() not .fmt("YYYY-MM-DD")
- Calendar awareness: add_months(1) handles edge cases properly
- UTC internal storage: Always UTC, timezone conversion for display
- Glang-native: Method chaining, functional programming, precision integration
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


# Extend TimeValue with time-specific methods
def extend_time_value():
    """Add time-specific methods to TimeValue class."""
    
    def as_date(self, format_preset: str = "iso") -> str:
        """Get date representation.
        
        Usage: time.as_date() -> "2025-01-15"
               time.as_date("long") -> "January 15, 2025"
        """
        dt = self.to_python_datetime()
        if format_preset in TimeModule.FORMAT_PRESETS:
            return dt.strftime(TimeModule.FORMAT_PRESETS[format_preset])
        else:
            # Custom format string
            return dt.strftime(format_preset)
    
    def as_time(self, format_preset: str = "24hour") -> str:
        """Get time representation.
        
        Usage: time.as_time() -> "14:30:45"
               time.as_time("12hour") -> "2:30:45 PM"
        """
        dt = self.to_python_datetime()
        if format_preset in TimeModule.FORMAT_PRESETS:
            return dt.strftime(TimeModule.FORMAT_PRESETS[format_preset])
        else:
            return dt.strftime(format_preset)
    
    def as_datetime(self, timezone: str = "utc") -> str:
        """Get datetime representation.
        
        Usage: time.as_datetime() -> "2025-01-15T14:30:45Z"
               time.as_datetime("EST") -> "2025-01-15T09:30:45-05:00"
        """
        dt = self.to_python_datetime()
        
        if timezone.upper() == "UTC":
            return dt.strftime("%Y-%m-%dT%H:%M:%SZ")
        elif timezone.upper() in TimeModule.TIMEZONES:
            # Convert to target timezone
            offset = TimeModule.TIMEZONES[timezone.upper()]
            local_dt = dt + python_datetime.timedelta(hours=offset)
            sign = "+" if offset >= 0 else "-"
            offset_str = f"{sign}{abs(offset):02d}:00"
            return local_dt.strftime(f"%Y-%m-%dT%H:%M:%S{offset_str}")
        elif timezone.lower() == "local":
            # Use system local timezone
            local_dt = dt.astimezone()
            return local_dt.strftime("%Y-%m-%dT%H:%M:%S%z")
        else:
            raise ValueError(f"Unknown timezone: {timezone}")
    
    def as_timestamp(self) -> float:
        """Get Unix timestamp.
        
        Usage: time.as_timestamp() -> 1704067200.123
        """
        return self.timestamp.to_python_float()
    
    # Advanced formatting
    def format(self, pattern: str) -> str:
        """Format with custom pattern.
        
        Usage: time.format("MMMM DD, YYYY at h:mm A") -> "January 15, 2025 at 2:30 PM"
        """
        dt = self.to_python_datetime()
        # Convert common format tokens to Python strftime
        python_pattern = pattern.replace("YYYY", "%Y").replace("MM", "%m").replace("DD", "%d")
        python_pattern = python_pattern.replace("MMMM", "%B").replace("MMM", "%b")
        python_pattern = python_pattern.replace("HH", "%H").replace("h", "%I")
        python_pattern = python_pattern.replace("mm", "%M").replace("ss", "%S")
        python_pattern = python_pattern.replace("A", "%p").replace("a", "%p")
        python_pattern = python_pattern.replace("dddd", "%A").replace("ddd", "%a")
        
        return dt.strftime(python_pattern)
    
    def fmt(self, pattern: str) -> str:
        """Alias for format()."""
        return self.format(pattern)
    
    # Calendar-aware arithmetic
    def add_seconds(self, n: Union[int, float]) -> 'TimeValue':
        """Add seconds and return new Time."""
        new_timestamp = self.timestamp.to_python_float() + n
        return TimeValue(new_timestamp, self.position)
    
    def add_minutes(self, n: Union[int, float]) -> 'TimeValue':
        """Add minutes and return new Time."""
        return self.add_seconds(n * 60)
    
    def add_hours(self, n: Union[int, float]) -> 'TimeValue':
        """Add hours and return new Time."""
        return self.add_seconds(n * 3600)
    
    def add_days(self, n: Union[int, float]) -> 'TimeValue':
        """Add days and return new Time."""
        return self.add_seconds(n * 86400)
    
    def add_weeks(self, n: Union[int, float]) -> 'TimeValue':
        """Add weeks and return new Time."""
        return self.add_seconds(n * 604800)
    
    def add_months(self, n: int) -> 'TimeValue':
        """Add months with calendar awareness."""
        dt = self.to_python_datetime()
        
        # Calculate target month and year
        total_months = dt.month + n
        target_year = dt.year + (total_months - 1) // 12
        target_month = ((total_months - 1) % 12) + 1
        
        # Handle day overflow (e.g., Jan 31 + 1 month = Feb 28/29)
        max_day = calendar.monthrange(target_year, target_month)[1]
        target_day = min(dt.day, max_day)
        
        new_dt = dt.replace(year=target_year, month=target_month, day=target_day)
        return TimeValue(new_dt.timestamp(), self.position)
    
    def add_years(self, n: int) -> 'TimeValue':
        """Add years with calendar awareness."""
        dt = self.to_python_datetime()
        target_year = dt.year + n
        
        # Handle leap year edge case (Feb 29 -> Feb 28 in non-leap year)
        if dt.month == 2 and dt.day == 29 and not calendar.isleap(target_year):
            target_day = 28
        else:
            target_day = dt.day
        
        new_dt = dt.replace(year=target_year, day=target_day)
        return TimeValue(new_dt.timestamp(), self.position)
    
    # Semantic navigation
    def start_of_day(self) -> 'TimeValue':
        """Get start of day (00:00:00)."""
        dt = self.to_python_datetime()
        start_dt = dt.replace(hour=0, minute=0, second=0, microsecond=0)
        return TimeValue(start_dt.timestamp(), self.position)
    
    def end_of_day(self) -> 'TimeValue':
        """Get end of day (23:59:59)."""
        dt = self.to_python_datetime()
        end_dt = dt.replace(hour=23, minute=59, second=59, microsecond=999999)
        return TimeValue(end_dt.timestamp(), self.position)
    
    def start_of_week(self) -> 'TimeValue':
        """Get start of week (Monday)."""
        dt = self.to_python_datetime()
        days_since_monday = dt.weekday()
        monday = dt - python_datetime.timedelta(days=days_since_monday)
        monday_start = monday.replace(hour=0, minute=0, second=0, microsecond=0)
        return TimeValue(monday_start.timestamp(), self.position)
    
    def end_of_week(self) -> 'TimeValue':
        """Get end of week (Sunday)."""
        dt = self.to_python_datetime()
        days_until_sunday = 6 - dt.weekday()
        sunday = dt + python_datetime.timedelta(days=days_until_sunday)
        sunday_end = sunday.replace(hour=23, minute=59, second=59, microsecond=999999)
        return TimeValue(sunday_end.timestamp(), self.position)
    
    def start_of_month(self) -> 'TimeValue':
        """Get start of month (1st day)."""
        dt = self.to_python_datetime()
        first_day = dt.replace(day=1, hour=0, minute=0, second=0, microsecond=0)
        return TimeValue(first_day.timestamp(), self.position)
    
    def end_of_month(self) -> 'TimeValue':
        """Get end of month (last day)."""
        dt = self.to_python_datetime()
        last_day = calendar.monthrange(dt.year, dt.month)[1]
        month_end = dt.replace(day=last_day, hour=23, minute=59, second=59, microsecond=999999)
        return TimeValue(month_end.timestamp(), self.position)
    
    def start_of_year(self) -> 'TimeValue':
        """Get start of year (Jan 1)."""
        dt = self.to_python_datetime()
        year_start = dt.replace(month=1, day=1, hour=0, minute=0, second=0, microsecond=0)
        return TimeValue(year_start.timestamp(), self.position)
    
    def end_of_year(self) -> 'TimeValue':
        """Get end of year (Dec 31).""" 
        dt = self.to_python_datetime()
        year_end = dt.replace(month=12, day=31, hour=23, minute=59, second=59, microsecond=999999)
        return TimeValue(year_end.timestamp(), self.position)
    
    # Predicates for functional programming
    def is_past(self) -> bool:
        """Check if time is in the past."""
        return self.timestamp.to_python_float() < time.time()
    
    def is_future(self) -> bool:
        """Check if time is in the future."""
        return self.timestamp.to_python_float() > time.time()
    
    def is_today(self) -> bool:
        """Check if time is today."""
        dt = self.to_python_datetime()
        today = python_datetime.date.today()
        return dt.date() == today
    
    def is_weekday(self) -> bool:
        """Check if time falls on a weekday (Mon-Fri)."""
        dt = self.to_python_datetime()
        return dt.weekday() < 5  # 0-4 are Mon-Fri
    
    def is_weekend(self) -> bool:
        """Check if time falls on weekend (Sat-Sun)."""
        return not self.is_weekday()
    
    def is_business_hour(self) -> bool:
        """Check if time is during business hours (9 AM - 5 PM)."""
        dt = self.to_python_datetime()
        return 9 <= dt.hour < 17 and self.is_weekday()
    
    def is_morning(self) -> bool:
        """Check if time is morning (5 AM - 12 PM)."""
        dt = self.to_python_datetime()
        return 5 <= dt.hour < 12
    
    def is_afternoon(self) -> bool:
        """Check if time is afternoon (12 PM - 6 PM)."""
        dt = self.to_python_datetime()
        return 12 <= dt.hour < 18
    
    def is_evening(self) -> bool:
        """Check if time is evening (6 PM - 10 PM)."""
        dt = self.to_python_datetime()
        return 18 <= dt.hour < 22
    
    # Calculations
    def days_since(self, other: 'TimeValue') -> float:
        """Calculate days since another time."""
        diff_seconds = self.timestamp.to_python_float() - other.timestamp.to_python_float()
        return diff_seconds / 86400
    
    def hours_since(self, other: 'TimeValue') -> float:
        """Calculate hours since another time."""
        diff_seconds = self.timestamp.to_python_float() - other.timestamp.to_python_float()
        return diff_seconds / 3600
    
    def years_until(self, other: 'TimeValue') -> float:
        """Calculate years until another time."""
        diff_seconds = other.timestamp.to_python_float() - self.timestamp.to_python_float()
        return diff_seconds / 31556952  # Average year in seconds


class TimeModule:
    """Built-in Time module providing a single Time type with natural methods."""
    
    # Timezone mappings (basic set - can be extended)
    TIMEZONES = {
        'UTC': 0, 'GMT': 0,
        'EST': -5, 'EDT': -4,
        'PST': -8, 'PDT': -7,
        'CST': -6, 'CDT': -5,
        'MST': -7, 'MDT': -6,
        'JST': 9,
        'CET': 1, 'CEST': 2,
    }
    
    # Format presets
    FORMAT_PRESETS = {
        'iso': '%Y-%m-%d',
        'short': '%m/%d/%y', 
        'long': '%B %d, %Y',
        '24hour': '%H:%M:%S',
        '12hour': '%I:%M:%S %p',
        'local': '%Y-%m-%dT%H:%M:%S%z',
        'utc': '%Y-%m-%dT%H:%M:%SZ',
    }
    
    @staticmethod
    def now(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get current time as Time value.
        
        Usage: time current = Time.now()
        """
        try:
            current_timestamp = time.time()
            return TimeValue(current_timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get current time: {str(e)}", position)
    
    @staticmethod
    def today(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get start of today (00:00:00 UTC).
        
        Usage: time today = Time.today()
        """
        try:
            today = python_datetime.date.today()
            today_datetime = python_datetime.datetime.combine(today, python_datetime.time.min, python_datetime.timezone.utc)
            timestamp = today_datetime.timestamp()
            return TimeValue(timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get today: {str(e)}", position)
    
    @staticmethod
    def tomorrow(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get start of tomorrow.
        
        Usage: time tomorrow = Time.tomorrow()
        """
        try:
            tomorrow = python_datetime.date.today() + python_datetime.timedelta(days=1)
            tomorrow_datetime = python_datetime.datetime.combine(tomorrow, python_datetime.time.min, python_datetime.timezone.utc)
            timestamp = tomorrow_datetime.timestamp()
            return TimeValue(timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get tomorrow: {str(e)}", position)
    
    @staticmethod
    def yesterday(position: Optional[SourcePosition] = None) -> GlangValue:
        """Get start of yesterday.
        
        Usage: time yesterday = Time.yesterday()
        """
        try:
            yesterday = python_datetime.date.today() - python_datetime.timedelta(days=1)
            yesterday_datetime = python_datetime.datetime.combine(yesterday, python_datetime.time.min, python_datetime.timezone.utc)
            timestamp = yesterday_datetime.timestamp()
            return TimeValue(timestamp, position)
        except Exception as e:
            raise RuntimeError(f"Failed to get yesterday: {str(e)}", position)
    
    @staticmethod
    def from_components(year: GlangValue, month: GlangValue, day: GlangValue,
                       hour: Optional[GlangValue] = None, minute: Optional[GlangValue] = None, 
                       second: Optional[GlangValue] = None, timezone: Optional[GlangValue] = None,
                       position: Optional[SourcePosition] = None) -> GlangValue:
        """Create Time from date/time components.
        
        Usage: 
            time birthday = Time(1990, 12, 25)
            time meeting = Time(2025, 1, 15, 14, 30, 0)
        """
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
            
            # Create datetime (assumed UTC unless timezone specified)
            dt = python_datetime.datetime(year_val, month_val, day_val, hour_val, minute_val, second_val, tzinfo=python_datetime.timezone.utc)
            
            # Handle timezone conversion if specified
            if timezone and isinstance(timezone, StringValue):
                tz_str = timezone.to_python().upper()
                if tz_str in TimeModule.TIMEZONES:
                    # Convert from specified timezone to UTC
                    tz_offset = TimeModule.TIMEZONES[tz_str]
                    dt = dt.replace(tzinfo=None) - python_datetime.timedelta(hours=tz_offset)
                    dt = dt.replace(tzinfo=python_datetime.timezone.utc)
                elif tz_str != 'UTC':
                    raise RuntimeError(f"Unknown timezone: {tz_str}", position)
            
            timestamp = dt.timestamp()
            return TimeValue(timestamp, position)
            
        except ValueError as e:
            raise RuntimeError(f"Invalid date/time: {str(e)}", position)
        except Exception as e:
            raise RuntimeError(f"Failed to create time: {str(e)}", position)
    
    @staticmethod
    def from_string(iso_string: GlangValue, timezone: Optional[GlangValue] = None,
                   position: Optional[SourcePosition] = None) -> GlangValue:
        """Create Time from ISO string.
        
        Usage:
            time t1 = Time("2025-01-15T14:30:00")
            time t2 = Time("2025-01-15T14:30:00", "EST")
        """
        try:
            if not isinstance(iso_string, StringValue):
                raise RuntimeError("Time string must be a string", position)
            
            time_str = iso_string.to_python()
            
            # Try to parse ISO format
            try:
                # Handle various ISO formats
                if 'T' in time_str:
                    if time_str.endswith('Z'):
                        dt = python_datetime.datetime.fromisoformat(time_str[:-1] + '+00:00')
                    elif '+' in time_str or time_str.count('-') > 2:
                        dt = python_datetime.datetime.fromisoformat(time_str)
                    else:
                        dt = python_datetime.datetime.fromisoformat(time_str).replace(tzinfo=python_datetime.timezone.utc)
                else:
                    # Date only - assume start of day UTC
                    date_obj = python_datetime.date.fromisoformat(time_str)
                    dt = python_datetime.datetime.combine(date_obj, python_datetime.time.min, python_datetime.timezone.utc)
                
                # Convert to UTC if needed
                if dt.tzinfo != python_datetime.timezone.utc:
                    dt = dt.astimezone(python_datetime.timezone.utc)
                
            except ValueError:
                raise RuntimeError(f"Invalid time format: {time_str}. Use ISO format like '2025-01-15T14:30:00'", position)
            
            # Handle timezone override
            if timezone and isinstance(timezone, StringValue):
                tz_str = timezone.to_python().upper()
                if tz_str in TimeModule.TIMEZONES and tz_str != 'UTC':
                    # Reinterpret the time as being in the specified timezone
                    tz_offset = TimeModule.TIMEZONES[tz_str]
                    dt = dt.replace(tzinfo=None) - python_datetime.timedelta(hours=tz_offset)
                    dt = dt.replace(tzinfo=python_datetime.timezone.utc)
            
            timestamp = dt.timestamp()
            return TimeValue(timestamp, position)
            
        except Exception as e:
            raise RuntimeError(f"Failed to parse time string: {str(e)}", position)
    
    @staticmethod
    def from_timestamp(timestamp: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Create Time from Unix timestamp.
        
        Usage: time t = Time.from_timestamp(1704067200)
        """
        try:
            if not isinstance(timestamp, NumberValue):
                raise RuntimeError("Timestamp must be a number", position)
            
            timestamp_val = timestamp.to_python()
            return TimeValue(timestamp_val, position)
            
        except Exception as e:
            raise RuntimeError(f"Failed to create time from timestamp: {str(e)}", position)
    
    # Time duration helpers (return seconds)
    @staticmethod
    def seconds(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert seconds to seconds (identity function for consistency)."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        return n
    
    @staticmethod  
    def minutes(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert minutes to seconds."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 60
        return NumberValue(create_glang_number(seconds), position)
    
    @staticmethod
    def hours(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert hours to seconds.""" 
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 3600
        return NumberValue(create_glang_number(seconds), position)
    
    @staticmethod
    def days(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert days to seconds."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 86400
        return NumberValue(create_glang_number(seconds), position)
    
    @staticmethod
    def weeks(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert weeks to seconds."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 604800
        return NumberValue(create_glang_number(seconds), position)
    
    @staticmethod
    def months(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert months to seconds (average month)."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 2629746  # Average month in seconds
        return NumberValue(create_glang_number(seconds), position)
    
    @staticmethod
    def years(n: GlangValue, position: Optional[SourcePosition] = None) -> GlangValue:
        """Convert years to seconds (average year)."""
        if not isinstance(n, NumberValue):
            raise RuntimeError("Duration must be a number", position)
        seconds = n.to_python() * 31556952  # Average year in seconds
        return NumberValue(create_glang_number(seconds), position)


def create_time_module_namespace():
    """Create the namespace for the built-in Time module."""
    from .module_builder import create_module

    return create_module(
        "time",
        functions={
            # Creation functions
            'now': TimeModule.now,
            'today': TimeModule.today,
            'tomorrow': TimeModule.tomorrow,
            'yesterday': TimeModule.yesterday,
            'from_components': TimeModule.from_components,
            'from_string': TimeModule.from_string,
            'from_timestamp': TimeModule.from_timestamp,

            # Duration helpers
            'seconds': TimeModule.seconds,
            'minutes': TimeModule.minutes,
            'hours': TimeModule.hours,
            'days': TimeModule.days,
            'weeks': TimeModule.weeks,
            'months': TimeModule.months,
            'years': TimeModule.years,
        }
    )