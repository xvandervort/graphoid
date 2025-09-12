# Date/Time Module Design for Glang

A comprehensive date/time module that embraces Glang's native semantics instead of wrapping Python's datetime library.

## Design Philosophy

The Glang date/time module should exemplify Glang's core principles:

1. **Data Node Consistency**: Dates and times as first-class data nodes
2. **Precision Context Integration**: Work seamlessly with precision blocks
3. **Functional Programming**: Chainable operations and transformations
4. **Immutability Support**: All date operations return new values
5. **Type Safety**: Clear distinction between dates, times, and datetimes
6. **Method Chaining**: Ruby-like fluent interfaces
7. **Semantic Predicates**: Human-readable filter operations

## Core Data Structures

### Date as Data Node
```glang
import "datetime" as dt

# Dates are data nodes, not objects
birthday = dt.date(1990, 12, 25)
# Returns: { "date": "1990-12-25" }

birthday.key()       # "date"
birthday.value()     # "1990-12-25"
birthday.type()      # "data"
```

### Time as Data Node
```glang
meeting_time = dt.time(14, 30, 0)
# Returns: { "time": "14:30:00" }

lunch_time = dt.time(12, 30)
# Returns: { "time": "12:30:00" }
```

### DateTime as Data Node
```glang
appointment = dt.datetime(2025, 1, 15, 14, 30, 0)
# Returns: { "datetime": "2025-01-15T14:30:00" }

# With timezone
meeting = dt.datetime(2025, 1, 15, 14, 30, 0, "UTC")
# Returns: { "datetime": "2025-01-15T14:30:00Z" }
```

### Duration as Data Node
```glang
work_hours = dt.duration(8, 30, 0)  # 8 hours, 30 minutes
# Returns: { "duration": "08:30:00" }

project_time = dt.duration(days: 30, hours: 4, minutes: 15)
# Returns: { "duration": "30d04:15:00" }
```

## Precision Context Integration

### Integer Precision (Unix Timestamps)
```glang
precision 0 {
    now = dt.now()           # 1704067200 (seconds since epoch)
    future = now + 3600      # 1704070800 (one hour later)
    
    # All timestamp arithmetic uses integers
    duration = future - now  # 3600 (seconds)
}
```

### Millisecond Precision
```glang
precision 3 {
    precise_now = dt.now()      # 1704067200.123 (with milliseconds)
    micro_delay = precise_now + 0.001  # Add 1 millisecond
}
```

### Nanosecond Precision (High-Performance Computing)
```glang
precision 9 {
    nano_now = dt.now()         # 1704067200.123456789
    benchmark_start = nano_now
    # ... do work ...
    elapsed = dt.now() - benchmark_start  # Nanosecond precision timing
}
```

## Functional Programming Integration

### Transformation Functions
```glang
dates = ["2025-01-01", "2025-02-01", "2025-03-01"]

# Convert strings to date data nodes
date_nodes = dates.map("to_date")
# Result: [{"date": "2025-01-01"}, {"date": "2025-02-01"}, {"date": "2025-03-01"}]

# Transform to weekdays
weekdays = date_nodes.map("to_weekday")  
# Result: ["Wednesday", "Saturday", "Saturday"]

# Add days
future_dates = date_nodes.map("add_30_days")
# Result: [{"date": "2025-01-31"}, {"date": "2025-03-03"}, {"date": "2025-04-01"}]
```

### Semantic Predicates
```glang
dates = [
    dt.date(2025, 1, 6),   # Monday
    dt.date(2025, 1, 7),   # Tuesday  
    dt.date(2025, 1, 11),  # Saturday
    dt.date(2025, 1, 12)   # Sunday
]

# Filter using semantic predicates
weekdays = dates.filter("is_weekday")
# Result: [Monday, Tuesday]

weekends = dates.filter("is_weekend")
# Result: [Saturday, Sunday]

business_days = dates.filter("is_business_day")
# Excludes weekends and holidays

this_month = dates.filter("is_current_month")
past_dates = dates.filter("is_past")
future_dates = dates.filter("is_future")
```

### Available Date Predicates
```glang
# Day of week predicates
"is_weekday", "is_weekend"
"is_monday", "is_tuesday", "is_wednesday", "is_thursday", "is_friday"
"is_saturday", "is_sunday"

# Time-based predicates
"is_past", "is_future", "is_today"
"is_this_week", "is_this_month", "is_this_year"
"is_last_week", "is_last_month", "is_last_year"
"is_next_week", "is_next_month", "is_next_year"

# Calendar predicates  
"is_business_day", "is_holiday"
"is_leap_year", "is_month_end", "is_month_start"
"is_quarter_end", "is_year_end"

# Season predicates (Northern Hemisphere by default)
"is_spring", "is_summer", "is_fall", "is_winter"
```

### Date Transformations
```glang
# Available transformations for map()
"to_weekday", "to_month_name", "to_year"
"to_start_of_week", "to_end_of_week"
"to_start_of_month", "to_end_of_month" 
"to_start_of_year", "to_end_of_year"

# Date arithmetic transformations
"add_day", "add_week", "add_month", "add_year"
"subtract_day", "subtract_week", "subtract_month", "subtract_year"
"add_7_days", "add_30_days", "add_90_days"

# Format transformations
"to_iso_string", "to_short_date", "to_long_date"
"to_timestamp", "to_unix_time"
```

## Method Chaining Interface

### Fluent Date Operations
```glang
result = dt.date(2025, 1, 15)
    .add_months(3)           # April 15, 2025
    .to_start_of_month()     # April 1, 2025
    .add_weeks(2)            # April 15, 2025
    .to_weekday()            # "Tuesday"

print(result)  # "Tuesday"
```

### Complex Date Calculations
```glang
quarterly_report = dt.today()
    .to_start_of_quarter()
    .subtract_quarters(1)     # Previous quarter
    .to_end_of_quarter()      # Last day of previous quarter
    
print(quarterly_report.format("YYYY-MM-DD"))
```

### Business Date Calculations
```glang
# Calculate next business day (skipping weekends and holidays)
next_work_day = dt.today()
    .add_business_days(1)
    .skip_holidays()
    
# Project deadline: 30 business days from now
deadline = dt.today()
    .add_business_days(30)
    .format("MMMM DD, YYYY")
```

## Time Zone Support

### Time Zone as Data Nodes
```glang
# Time zones are also data nodes
eastern = dt.timezone("America/New_York")
# Returns: { "timezone": "America/New_York" }

utc = dt.timezone("UTC")
pacific = dt.timezone("America/Los_Angeles")
```

### Time Zone Conversion
```glang
utc_time = dt.datetime(2025, 1, 15, 20, 0, 0, "UTC")
eastern_time = utc_time.to_timezone("America/New_York")
# Returns: { "datetime": "2025-01-15T15:00:00-05:00" }

# Method chaining with time zones
meeting_times = [
    dt.datetime(2025, 1, 15, 14, 0, 0, "UTC")
        .to_timezone("America/New_York"),
    dt.datetime(2025, 1, 15, 14, 0, 0, "UTC") 
        .to_timezone("America/Los_Angeles"),
    dt.datetime(2025, 1, 15, 14, 0, 0, "UTC")
        .to_timezone("Asia/Tokyo")
]
```

## Formatting and Parsing

### Format Tokens (Inspired by Moment.js but Glang-native)
```glang
date = dt.date(2025, 1, 15)

# Basic formatting
date.format("YYYY-MM-DD")      # "2025-01-15"
date.format("MMM DD, YYYY")    # "Jan 15, 2025" 
date.format("dddd, MMMM Do")   # "Wednesday, January 15th"

# Custom formats with precision
precision 0 {
    timestamp = dt.now().format("X")  # Unix timestamp: "1704067200"
}

precision 3 {
    precise = dt.now().format("X.SSS")  # With milliseconds: "1704067200.123"
}
```

### Parsing Strings to Dates
```glang
# Parse common formats
date1 = dt.parse("2025-01-15")           # ISO format
date2 = dt.parse("01/15/2025", "MM/DD/YYYY")  # US format
date3 = dt.parse("15-Jan-2025", "DD-MMM-YYYY") # European format

# Flexible parsing
flexible = dt.parse_flexible("January 15th, 2025")
# Uses intelligent parsing for human-readable dates
```

## Duration and Interval Operations

### Duration Arithmetic
```glang
start_time = dt.datetime(2025, 1, 15, 9, 0, 0)
end_time = dt.datetime(2025, 1, 15, 17, 30, 0)

work_duration = end_time.subtract(start_time)
# Returns: { "duration": "08:30:00" }

# Duration methods
work_duration.hours()      # 8.5
work_duration.minutes()    # 510
work_duration.seconds()    # 30600

# Human readable
work_duration.humanize()   # "8 hours and 30 minutes"
```

### Date Ranges and Intervals
```glang
# Create date ranges
project_period = dt.range(
    dt.date(2025, 1, 1),
    dt.date(2025, 3, 31)
)

# Check if date falls in range
milestone = dt.date(2025, 2, 15)
is_during_project = project_period.contains(milestone)  # true

# Generate dates in range
project_dates = project_period.to_list()
weekdays_only = project_dates.filter("is_weekday")
```

## Calendar Integration

### Holiday Support
```glang
# Define holiday calendar
us_holidays = dt.calendar("US")

independence_day = dt.date(2025, 7, 4)
is_holiday = us_holidays.is_holiday(independence_day)  # true

# Next business day (considering holidays)
next_work = dt.today().add_business_days(1, us_holidays)
```

### Custom Calendars
```glang
# Define custom business calendar
company_calendar = dt.calendar("custom")
    .add_holiday(dt.date(2025, 12, 24))  # Christmas Eve
    .add_holiday(dt.date(2025, 12, 31))  # New Year's Eve
    .set_business_days(["monday", "tuesday", "wednesday", "thursday", "friday"])

# Use with business day calculations
project_days = dt.date(2025, 1, 1)
    .add_business_days(30, company_calendar)
```

## Immutability Integration

### Frozen Dates
```glang
contract_date = dt.date(2025, 1, 15).freeze()
contract_date.is_frozen()  # true

# All operations return new dates
extended_date = contract_date.add_months(12)  # New date, original unchanged
contract_date.value()      # Still "2025-01-15"
extended_date.value()      # "2026-01-15"
```

### Contamination Rules
```glang
frozen_dates = [
    dt.date(2025, 1, 1).freeze(),
    dt.date(2025, 2, 1).freeze()
]

mutable_dates = [
    dt.date(2025, 3, 1),
    dt.date(2025, 4, 1)
]

# Cannot mix frozen and unfrozen dates
# mutable_dates.append(frozen_dates[0])  # Error: contamination violation

if mutable_dates.can_accept(frozen_dates[0]) {
    mutable_dates.append(frozen_dates[0])
} else {
    print("Cannot mix frozen and unfrozen dates")
}
```

## Module API Design

### Core Functions
```glang
# Current date/time
dt.now()                    # Current datetime
dt.today()                  # Current date
dt.current_time()           # Current time

# Creation functions
dt.date(year, month, day)
dt.time(hour, minute, second)
dt.datetime(year, month, day, hour, minute, second, timezone)
dt.duration(hours, minutes, seconds)
dt.duration(days: N, hours: N, minutes: N, seconds: N)

# Parsing functions  
dt.parse(date_string, format)
dt.parse_flexible(human_readable_string)
dt.from_timestamp(unix_timestamp)
dt.from_iso(iso_string)

# Utility functions
dt.is_valid_date(year, month, day)
dt.days_in_month(year, month)
dt.is_leap_year(year)
dt.timezone_list()          # Available timezones
```

### Constants and Lookups
```glang
# Month names
dt.MONTHS           # ["January", "February", ..., "December"]
dt.MONTHS_SHORT     # ["Jan", "Feb", ..., "Dec"]

# Weekday names
dt.WEEKDAYS         # ["Monday", "Tuesday", ..., "Sunday"]
dt.WEEKDAYS_SHORT   # ["Mon", "Tue", ..., "Sun"]

# Common timezones
dt.UTC
dt.LOCAL            # System local timezone
```

## Performance Considerations

### Efficient Internal Representation
- **Unix timestamps** for internal storage (precision-aware)
- **Lazy parsing** of strings only when needed
- **Cached calculations** for expensive operations (timezone conversions)
- **Minimal object creation** through data node reuse

### Precision-Optimized Operations
```glang
# Low precision for performance-critical code
precision 0 {
    list timestamps = []
    for i in range(100000) {
        timestamps.append(dt.now())  # Fast integer timestamps
    }
}

# High precision only when needed
precision 6 {
    start = dt.now()
    # ... precise timing operation ...
    elapsed = dt.now() - start  # Microsecond precision
}
```

## Error Handling

### Validation with Clear Messages
```glang
# Invalid date handling
invalid_date = dt.date(2025, 2, 30)
# Error: "Invalid date: February only has 28 days in 2025"

invalid_time = dt.time(25, 0, 0)
# Error: "Invalid time: hour must be between 0 and 23, got 25"

# Timezone validation
invalid_tz = dt.datetime(2025, 1, 15, 12, 0, 0, "Invalid/Timezone")
# Error: "Unknown timezone: Invalid/Timezone. Use dt.timezone_list() to see available timezones"
```

### Graceful Degradation
```glang
# Flexible parsing with fallbacks
date_or_none = dt.try_parse("not a date")  # Returns none instead of error
if date_or_none != none {
    print("Parsed successfully: " + date_or_none.format("YYYY-MM-DD"))
} else {
    print("Could not parse date")
}
```

## Integration Examples

### With Control Flow
```glang
for date in project_dates {
    if date.is_business_day() && !date.is_holiday() {
        scheduled_tasks.append(create_task(date))
    }
}

# Nested precision contexts
precision 0 {  # Day-level precision
    daily_timestamps = []
    for day in date_range {
        precision 3 {  # Millisecond precision for timing
            start_time = dt.now()
            process_daily_data(day)
            elapsed = dt.now() - start_time
            daily_timestamps.append({
                "date": day,
                "processing_time": elapsed
            })
        }
    }
}
```

### With Functional Programming
```glang
# Complex date processing pipeline
quarterly_reports = all_dates
    .filter("is_quarter_end")
    .map("to_quarter_name")
    .map("generate_report")
    .each("send_notification")

# Business day calculations
work_schedule = project_start_date
    .generate_range(project_end_date)
    .filter("is_business_day")
    .reject("is_holiday")
    .map("assign_tasks")
```

## Future Extensibility

### Plugin Architecture for Calendars
```glang
# Custom calendar systems could be added
islamic_calendar = dt.calendar("hijri")
hebrew_calendar = dt.calendar("hebrew")
fiscal_calendar = dt.calendar("fiscal", fiscal_year_start: "october")
```

### Integration with Graph Features (Phase 2)
```glang
# Future: Date relationships as graph edges
project_timeline = {
    "milestone_1": dt.date(2025, 2, 1),
    "milestone_2": dt.date(2025, 3, 15),
    dependencies: {
        "milestone_2".depends_on("milestone_1")  # Graph relationship
    }
}
```

---

This design creates a date/time module that feels native to Glang while providing powerful, intuitive functionality for real-world applications. The integration with precision blocks, functional programming, and data nodes makes it uniquely Glang-like rather than a simple wrapper around existing libraries.