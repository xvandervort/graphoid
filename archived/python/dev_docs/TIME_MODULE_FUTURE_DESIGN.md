# Time Module Redesign - Single Type, Maximum Simplicity

A complete redesign of Glang's time handling with one simple principle: **One Time type, many natural representations**.

## Core Philosophy

- **Single Type**: Everything is a `Time` (internally a UTC timestamp)
- **Natural Methods**: `Time.now.as_date` not `Time.now.fmt("YYYY-MM-DD")`
- **Calendar Awareness**: `add_months(1)` means next calendar month
- **UTC Internal**: Always UTC internally, timezone conversion for display
- **Immutable**: All operations return new Time instances
- **Glang Native**: Method chaining, functional programming, precision integration

## Basic Usage

### Time Creation
```glang
time now = Time.now()                    # Current time
time today = Time.today()                # Start of today (00:00:00 UTC)
time birthday = Time(1990, 12, 25)       # From date components
time meeting = Time("2025-01-15T14:30:00")  # From ISO string
time timestamp = Time(1704067200)        # From Unix timestamp
```

### Natural Representations
```glang
time t = Time.now()

# Natural method names instead of format strings
t.as_date()              # "2025-01-15"
t.as_time()              # "14:30:45"  
t.as_datetime()          # "2025-01-15T14:30:45Z"
t.as_timestamp()         # 1704067200.123

# With optional formatting
t.as_date("long")        # "January 15, 2025"
t.as_date("short")       # "01/15/25"
t.as_time("12hour")      # "2:30:45 PM"
t.as_datetime("local")   # "2025-01-15T09:30:45-05:00" (EST)
```

### Advanced Formatting (when needed)
```glang
# Keep format() for complex cases, fmt() as alias
t.format("MMMM DD, YYYY at h:mm A")  # "January 15, 2025 at 2:30 PM" 
t.fmt("dddd")                        # "Tuesday" (alias)
```

## Calendar-Aware Date Arithmetic

### Addition Methods
```glang
time start = Time(2025, 1, 31)  # January 31st

# Calendar-aware arithmetic
next_month = start.add_months(1)     # February 28, 2025 (no Feb 31st)
next_year = start.add_years(1)       # January 31, 2026

# Always predictable
march = Time(2025, 1, 31).add_months(2)  # March 31, 2025 (exists)

# Time arithmetic  
later = start.add_days(5)
soon = start.add_hours(3)
moment = start.add_seconds(30)
```

### Semantic Constants
```glang
# Helper functions return seconds for arithmetic
time future = Time.now() + Time.days(1)      # Tomorrow
time past = Time.now() - Time.hours(2)       # 2 hours ago
time deadline = Time.now() + Time.weeks(3)   # 3 weeks from now

# Available helpers
Time.seconds(n)
Time.minutes(n) 
Time.hours(n)
Time.days(n)
Time.weeks(n)
Time.months(n)      # Average month in seconds
Time.years(n)       # Average year in seconds
```

## Timezone Handling

### Internal UTC, Display Local
```glang
time utc_time = Time.now()               # Always UTC internally

# Timezone conversion for display
utc_time.as_datetime("EST")              # "2025-01-15T09:30:45-05:00"
utc_time.as_datetime("PST")              # "2025-01-15T06:30:45-08:00"
utc_time.as_datetime("local")            # Uses system timezone

# Creation with timezone (converted to UTC internally)
time meeting = Time("2025-01-15T14:30:00", "EST")  # Stored as UTC
```

### Timezone Methods
```glang
time t = Time.now()

# Query timezone info
t.in_timezone("EST").as_datetime()       # Convert for display
t.timezone_offset("PST")                 # -8 (hours from UTC)
t.is_dst("EST")                          # true/false if in daylight saving
```

## Method Chaining and Fluent Interface

### Business Logic Chains
```glang
# Project deadline: 30 business days from now
deadline = Time.now()
    .add_days(30)
    .skip_weekends()
    .skip_holidays()
    .as_date("long")

# Birthday calculations  
age_at_event = birthday
    .years_until(event_date)
    .as_string() + " years old"

# Meeting scheduling
next_meeting = last_meeting
    .add_weeks(2)
    .start_of_week()
    .add_hours(10)     # 10 AM Monday
    .as_datetime("local")
```

### Calendar Navigation
```glang
time t = Time(2025, 1, 15)  # January 15th

# Semantic navigation
t.start_of_day()       # 2025-01-15T00:00:00Z
t.end_of_day()         # 2025-01-15T23:59:59Z
t.start_of_week()      # Monday of this week
t.end_of_week()        # Sunday of this week  
t.start_of_month()     # 2025-01-01T00:00:00Z
t.start_of_year()      # 2025-01-01T00:00:00Z

# Chain them
quarter_end = t.start_of_year()
    .add_months(3)
    .add_days(-1)
    .end_of_day()      # Last second of Q1
```

## Functional Programming Integration

### Time Lists and Transformations
```glang
appointments = [
    Time("2025-01-15T09:00:00"),
    Time("2025-01-16T14:00:00"), 
    Time("2025-01-17T11:00:00"),
    Time("2025-01-18T16:00:00")
]

# Semantic predicates
weekday_appointments = appointments.filter("is_weekday")
morning_appointments = appointments.filter("is_morning") 
business_hours = appointments.filter("is_business_hour")
past_appointments = appointments.filter("is_past")

# Transformations
next_week = appointments.map("add_week")
date_strings = appointments.map("as_date")
timestamps = appointments.map("as_timestamp")

# Custom transformations with lambdas
rescheduled = appointments.map(t => t.add_days(7).start_of_day())
```

### Available Predicates
```glang
# Time-based predicates for filter()
"is_past", "is_future", "is_today"
"is_yesterday", "is_tomorrow"
"is_this_week", "is_this_month", "is_this_year"
"is_weekday", "is_weekend"
"is_morning", "is_afternoon", "is_evening"
"is_business_hour", "is_holiday"
"is_dst"  # In daylight saving time
```

### Available Transformations
```glang
# Time transformations for map()
"as_date", "as_time", "as_datetime", "as_timestamp"
"start_of_day", "end_of_day", "start_of_week", "end_of_week"
"add_day", "add_week", "add_month", "add_year"
"subtract_day", "subtract_week", "subtract_month"
"skip_weekends", "skip_holidays"
"to_utc", "to_local"
```

## Precision Context Integration

### Timestamp Precision
```glang
# Different precision for different needs
precision 0 {
    num timestamp = Time.now().as_timestamp()  # 1704067200 (integer seconds)
}

precision 3 {
    num timestamp = Time.now().as_timestamp()  # 1704067200.123 (milliseconds)
}

precision 6 {
    num timestamp = Time.now().as_timestamp()  # 1704067200.123456 (microseconds)
}
```

### Date Calculations with Precision
```glang
# Financial calculations (daily interest)
precision 8 {
    time start = Time(2025, 1, 1)
    time end = Time(2025, 12, 31)
    num days = end.days_since(start)       # Precise day calculation
    num daily_rate = annual_rate / days    # High-precision division
}
```

## Immutability Integration

### Frozen Times
```glang
time contract_date = Time(2025, 1, 15).freeze()
contract_date.is_frozen()  # true

# All operations return new times
time extended = contract_date.add_years(1)  # New time, original unchanged
contract_date.as_date()    # Still "2025-01-15"
extended.as_date()         # "2026-01-15"
```

### Contamination Rules
```glang
frozen_times = [
    Time(2025, 1, 1).freeze(),
    Time(2025, 2, 1).freeze()
]

mutable_times = [Time.now(), Time.tomorrow()]

# Cannot mix frozen and unfrozen times
if mutable_times.can_accept(frozen_times[0]) {
    mutable_times.append(frozen_times[0])
} else {
    print("Cannot mix frozen and unfrozen times")
}
```

## Implementation Architecture

### Time Class Structure
```glang
class Time extends GlangValue {
    # Internal representation
    _timestamp: GlangNumber     # UTC timestamp with precision support
    
    # Constructors
    static now() -> Time
    static today() -> Time
    static tomorrow() -> Time
    static yesterday() -> Time
    
    # Creation from components
    Time(year: num, month: num, day: num) -> Time
    Time(year: num, month: num, day: num, hour: num, min: num, sec: num) -> Time  
    Time(iso_string: string) -> Time
    Time(iso_string: string, timezone: string) -> Time
    Time(timestamp: num) -> Time
    
    # Natural representations
    as_date(format: string = "iso") -> string
    as_time(format: string = "24hour") -> string
    as_datetime(timezone: string = "utc") -> string
    as_timestamp() -> num
    
    # Advanced formatting  
    format(pattern: string) -> string
    fmt(pattern: string) -> string          # Alias for format
    
    # Calendar-aware arithmetic
    add_seconds(n: num) -> Time
    add_minutes(n: num) -> Time
    add_hours(n: num) -> Time
    add_days(n: num) -> Time
    add_weeks(n: num) -> Time
    add_months(n: num) -> Time              # Calendar aware
    add_years(n: num) -> Time               # Calendar aware
    
    # Semantic navigation
    start_of_day() -> Time
    end_of_day() -> Time
    start_of_week() -> Time
    end_of_week() -> Time
    start_of_month() -> Time
    end_of_month() -> Time
    start_of_year() -> Time
    end_of_year() -> Time
    
    # Timezone operations
    in_timezone(tz: string) -> Time
    timezone_offset(tz: string) -> num
    is_dst(tz: string) -> bool
    
    # Predicates (for functional programming)
    is_past() -> bool
    is_future() -> bool
    is_today() -> bool
    is_weekday() -> bool
    is_weekend() -> bool
    is_business_hour() -> bool
    is_morning() -> bool
    is_afternoon() -> bool
    is_evening() -> bool
    
    # Calculations
    days_since(other: Time) -> num
    hours_since(other: Time) -> num
    years_until(other: Time) -> num
    
    # Business logic helpers
    skip_weekends() -> Time
    skip_holidays() -> Time
    next_business_day() -> Time
}
```

### Helper Constants
```glang
# Time duration helpers (return seconds)
Time.seconds(n: num) -> num
Time.minutes(n: num) -> num  
Time.hours(n: num) -> num
Time.days(n: num) -> num
Time.weeks(n: num) -> num
Time.months(n: num) -> num    # Average month
Time.years(n: num) -> num     # Average year
```

## Key Design Decisions

### 1. Calendar-Aware Month/Year Arithmetic
```glang
# Smart handling of edge cases
Time(2025, 1, 31).add_months(1)   # Feb 28, 2025 (no Feb 31st)
Time(2024, 2, 29).add_years(1)    # Feb 28, 2025 (2025 not leap year)
Time(2025, 3, 31).add_months(-1)  # Feb 28, 2025 (backwards too)
```

### 2. UTC Internal Storage
- All times stored as UTC timestamps internally
- Timezone conversion only for display/parsing
- Eliminates DST and timezone arithmetic complexity
- Consistent comparison and arithmetic

### 3. Natural Method Names
- `as_date()` instead of `fmt("YYYY-MM-DD")`
- `start_of_month()` instead of `fmt("YYYY-MM-01")`  
- `is_weekday()` instead of complex date logic
- Readable by non-experts

### 4. Immutable by Default
- All arithmetic returns new Time instances
- Original times never modified
- Plays well with Glang's freeze/contamination system
- Thread-safe and predictable

## Examples

### Complete Time Processing Pipeline
```glang
import "time"

# Project scheduling
project_start = Time(2025, 2, 1)
milestones = [
    project_start.add_weeks(2),
    project_start.add_months(1), 
    project_start.add_months(2),
    project_start.add_months(3)
]

# Filter to business days and format
business_milestones = milestones
    .map("skip_weekends")
    .map("skip_holidays") 
    .filter("is_business_hour")
    .map("as_date", "long")

for milestone in business_milestones {
    print("Milestone: " + milestone)
}

# Timezone-aware meeting scheduling
meeting_utc = Time("2025-01-15T19:00:00")  # 7 PM UTC
print("Meeting times:")
print("  New York: " + meeting_utc.as_datetime("EST"))
print("  London: " + meeting_utc.as_datetime("GMT"))  
print("  Tokyo: " + meeting_utc.as_datetime("JST"))

# Performance timing with precision
precision 6 {
    time start = Time.now()
    # ... do work ...
    time end = Time.now()
    num elapsed = end.as_timestamp() - start.as_timestamp()
    print("Elapsed: " + elapsed.to_string() + " seconds")
}
```

---

This design achieves **maximum simplicity** with **maximum power** - one type that naturally handles all time needs while leveraging Glang's unique features. No more choosing between date, time, and datetime types!