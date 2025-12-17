# Plan: Showcase Class-Like Graphs (CLGs)

**Created:** December 15, 2025
**Purpose:** Put CLGs through their paces with stdlib rewrites, sample projects, and documentation

---

## Background

Class-Like Graphs (CLGs) are now feature-complete through Phase 23 plus semantic edges. All core features work:
- Graph declaration syntax (`graph Name { }`)
- Properties with implicit self
- Computed properties (`get`)
- Private methods (`priv`)
- Inheritance (`from`)
- Static methods, setters, mixins
- Type checking (`is_a()`, `responds_to()`)
- **Semantic edges** (method-property relationships, dependency tracking, inheritance as graph structure)

Now it's time to showcase these features.

---

## Part 1: Stdlib Modules as CLGs

### Best Candidates

| Module | Current | Proposed CLG | Benefit |
|--------|---------|--------------|---------|
| `statistics.gr` | 20 standalone functions | `DataSet` CLG | Store data, compute stats on demand |
| `time.gr` | Functions + constants | `DateTime` CLG | Timestamp state, computed properties |

### 1A. DataSet CLG (`stdlib/dataset.gr`)

**Design:**
```graphoid
graph DataSet {
    data: []  # Internal storage

    fn new(values) {
        instance = self.clone()
        instance.data = values
        return instance
    }

    # Computed properties leverage semantic edges!
    get count() { return data.length() }
    get sum() { return calculate_sum() }
    get mean() { return sum / count }
    get variance() { return calculate_variance() }
    get std_dev() { return variance.sqrt() }
    get min() { return find_min() }
    get max() { return find_max() }
    get range() { return max - min }

    # Methods
    fn add(value) {
        data = data.append(value)
        return self
    }

    fn summary() {
        return {
            "count": count,
            "mean": mean,
            "std_dev": std_dev,
            "min": min,
            "max": max
        }
    }

    fn quantile(q) { ... }

    # Private helpers
    priv fn calculate_sum() {
        total = 0
        for value in data {
            total = total + value
        }
        return total
    }

    priv fn calculate_variance() {
        m = mean
        sum_sq = 0
        for value in data {
            diff = value - m
            sum_sq = sum_sq + (diff * diff)
        }
        return sum_sq / count
    }

    priv fn find_min() {
        if data.length() == 0 { return none }
        result = data[0]
        for value in data {
            if value < result { result = value }
        }
        return result
    }

    priv fn find_max() {
        if data.length() == 0 { return none }
        result = data[0]
        for value in data {
            if value > result { result = value }
        }
        return result
    }
}
```

**Shows off:**
- Computed properties with `get`
- Private methods with `priv`
- Semantic dependency edges (mean depends on sum and count, std_dev depends on variance)
- Method chaining with `add()`

**Usage example:**
```graphoid
import "dataset"

ds = DataSet.new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print("Count: " + ds.count.to_string())
print("Mean: " + ds.mean.to_string())
print("Std Dev: " + ds.std_dev.to_string())
print("Summary: " + ds.summary().to_string())

# Analyze dependencies
print("Dependencies of 'mean': " + ds.dependencies("mean").to_string())
print("Properties in order: " + ds.dependency_order().to_string())
```

### 1B. DateTime CLG (`stdlib/datetime.gr`)

**Design:**
```graphoid
import "os"

SECONDS_PER_DAY = 86400
SECONDS_PER_HOUR = 3600
SECONDS_PER_MINUTE = 60
DAYS_IN_MONTH = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

graph DateTime {
    timestamp: 0  # Unix timestamp (seconds since 1970)

    # Constructor
    fn new(ts) {
        instance = self.clone()
        instance.timestamp = ts
        return instance
    }

    # Static factory methods
    static fn now() {
        return DateTime.new(os.system_timestamp())
    }

    static fn from_date(year, month, day) {
        # Calculate days since epoch
        days = days_since_epoch(year, month, day)
        return DateTime.new(days * SECONDS_PER_DAY)
    }

    # Computed properties (these create dependency edges!)
    get year() { return extract_year() }
    get month() { return extract_month() }
    get day() { return extract_day() }
    get hour() { return ((timestamp % SECONDS_PER_DAY) / SECONDS_PER_HOUR).down() }
    get minute() { return (((timestamp % SECONDS_PER_DAY) % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE).down() }
    get second() { return ((timestamp % SECONDS_PER_DAY) % SECONDS_PER_MINUTE).down() }

    get day_of_week() {
        days = (timestamp / SECONDS_PER_DAY).down()
        return (days + 3) % 7  # 1970-01-01 was Thursday (3)
    }

    get is_weekend() { return day_of_week >= 5 }
    get is_weekday() { return day_of_week < 5 }

    # Methods
    fn add_days(n) {
        timestamp = timestamp + (n * SECONDS_PER_DAY)
        return self
    }

    fn add_hours(n) {
        timestamp = timestamp + (n * SECONDS_PER_HOUR)
        return self
    }

    fn format(style) {
        if style == :date {
            return year.to_string() + "-" + pad(month) + "-" + pad(day)
        }
        if style == :time {
            return pad(hour) + ":" + pad(minute) + ":" + pad(second)
        }
        # Default: datetime
        return format(:date) + "T" + format(:time) + "Z"
    }

    # Private helpers
    priv fn pad(n) {
        if n < 10 { return "0" + n.to_string() }
        return n.to_string()
    }

    priv fn extract_year() { ... }
    priv fn extract_month() { ... }
    priv fn extract_day() { ... }
    priv fn days_since_epoch(year, month, day) { ... }
}
```

**Shows off:**
- Static methods (`DateTime.now()`, `DateTime.from_date()`)
- Computed property chains (is_weekend depends on day_of_week)
- Method chaining with fluent interface

---

## Part 2: Sample Projects

Create new directory: `samples/06-projects/`

### 2A. Climate Analyzer (`samples/06-projects/climate/`)

**Description:** Analyze 144 years of NASA global temperature data using CLGs and DataSet.

**File structure:**
```
samples/06-projects/climate/
├── README.md                    # Project documentation
├── data/
│   └── global_temperature.json  # NASA GISTEMP data (1880-2023), public domain
├── climate_analyzer.gr          # ClimateAnalyzer CLG
└── main.gr                      # Demo script
```

**Data:** Uses `data/global_temperature.json` - NASA GISTEMP data, public domain.

**Full implementation:**
```graphoid
# Climate Analyzer - Statistical Analysis of Global Temperature Data
# Demonstrates: CLG with computed properties, DataSet integration, semantic edges
# Data: NASA GISTEMP global temperature anomalies (degrees C vs 1951-1980 baseline)

import "dataset"
import "json"
import "io"

graph ClimateAnalyzer {
    years: []
    anomalies: []
    name: "Climate Data"

    fn new(dataset_name) {
        instance = self.clone()
        instance.name = dataset_name
        instance.years = []
        instance.anomalies = []
        return instance
    }

    fn load_data(filepath) {
        content = io.read(filepath)
        parsed = json.parse(content)
        records = parsed["data"]

        for record in records {
            years = years.append(record["year"])
            anomalies = anomalies.append(record["anomaly"])
        }
        return self
    }

    fn add_reading(year, anomaly) {
        years = years.append(year)
        anomalies = anomalies.append(anomaly)
        return self
    }

    # Computed properties - these create semantic edges!
    get count() { return anomalies.length() }
    get first_year() {
        if count == 0 { return none }
        return years[0]
    }
    get last_year() {
        if count == 0 { return none }
        return years[count - 1]
    }
    get earliest() {
        if count == 0 { return none }
        return anomalies[0]
    }
    get latest() {
        if count == 0 { return none }
        return anomalies[count - 1]
    }
    get total_change() {
        if count < 2 { return 0 }
        return latest - earliest
    }

    # Statistical analysis using DataSet
    fn statistics() {
        ds = DataSet.new(anomalies)
        return ds.summary()
    }

    fn decade_average(start_year) {
        # Calculate average anomaly for a decade
        decade_values = []
        i = 0
        while i < count {
            if years[i] >= start_year && years[i] < start_year + 10 {
                decade_values = decade_values.append(anomalies[i])
            }
            i = i + 1
        }

        if decade_values.length() == 0 { return none }
        ds = DataSet.new(decade_values)
        return ds.mean
    }

    fn warmest_year() {
        ds = DataSet.new(anomalies)
        max_anomaly = ds.max
        i = 0
        while i < count {
            if anomalies[i] == max_anomaly {
                return {"year": years[i], "anomaly": max_anomaly}
            }
            i = i + 1
        }
        return none
    }

    fn coldest_year() {
        ds = DataSet.new(anomalies)
        min_anomaly = ds.min
        i = 0
        while i < count {
            if anomalies[i] == min_anomaly {
                return {"year": years[i], "anomaly": min_anomaly}
            }
            i = i + 1
        }
        return none
    }

    fn volatility() {
        ds = DataSet.new(anomalies)
        return ds.std_dev
    }

    fn report() {
        print("=== " + name + " ===")
        print("Period: " + first_year.to_string() + " - " + last_year.to_string())
        print("Data points: " + count.to_string())
        print("")

        print("Temperature Change:")
        print("  First reading (" + first_year.to_string() + "): " + earliest.to_string() + " C")
        print("  Last reading (" + last_year.to_string() + "): " + latest.to_string() + " C")
        print("  Total change: +" + total_change.to_string() + " C")
        print("")

        warmest = warmest_year()
        coldest = coldest_year()
        print("Extremes:")
        print("  Warmest: " + warmest["year"].to_string() + " at +" + warmest["anomaly"].to_string() + " C")
        print("  Coldest: " + coldest["year"].to_string() + " at " + coldest["anomaly"].to_string() + " C")
        print("")

        print("Decade Averages:")
        print("  1880s: " + decade_average(1880).to_string() + " C")
        print("  1920s: " + decade_average(1920).to_string() + " C")
        print("  1960s: " + decade_average(1960).to_string() + " C")
        print("  2000s: " + decade_average(2000).to_string() + " C")
        print("  2010s: " + decade_average(2010).to_string() + " C")
        print("")

        print("Variability (std dev): " + volatility().to_string() + " C")
        print("")

        # Show semantic edges
        print("Computed property dependencies:")
        print("  total_change depends on: " + self.dependencies("total_change").to_string())
    }
}

# === Demo ===
print("=== Global Temperature Analysis ===")
print("Data: NASA GISTEMP (Public Domain)")
print("")

# Load real data
climate = ClimateAnalyzer.new("Global Temperature Anomalies")
climate.load_data("data/global_temperature.json")

# Generate report
climate.report()

# Show the graph structure
print("=== Graph Structure Analysis ===")
print("Methods that read 'anomalies': " + climate.property_readers("anomalies").to_string())
print("Methods that read 'years': " + climate.property_readers("years").to_string())
```

### 2B. Elevator System (Multiple Files, Multiple CLGs)

**Description:** Elevator simulator - a classic interview problem that showcases:
- Multiple CLGs composing together
- Graph-based data structures (Queue as a graph)
- Realistic subsystems (request queue, emergency phone, door controller)
- File separation (library CLGs vs demo)
- Emergency states (fire mode, power failure, obstruction)

**File structure:**
```
samples/06-projects/elevator/
├── README.md              # Project documentation
├── queue.gr               # Queue CLG - graph-based FIFO
├── call_box.gr            # Emergency phone CLG
├── elevator.gr            # Main Elevator CLG (imports Queue, CallBox)
└── main.gr                # Interactive demo
```

**Interactive mode** - The elevator demo runs as an interactive session:
```
$ graphoid samples/06-projects/elevator/main.gr

=== Elevator Simulator ===
Type 'help' for commands.

> request 5
  Request queued: floor 5
> request 8
  Request queued: floor 8
> step
  Moving up...
> step
  Floor 2
> status
  Floor: 2 | State: moving_up | Queue: [5, 8]
> run
  Floor 3
  Floor 4
  Floor 5
  Stopping - opening doors
  Doors open
  ...
> fire
  !!! FIRE ALARM - RETURNING TO GROUND FLOOR !!!
> help
  Commands: request N, step, run, status, fire, emergency, reset, phone, quit
> quit
```

---

#### 2B-1. Queue CLG (`samples/06-projects/elevator/queue.gr`)

**A queue IS a graph** - this is "everything is a graph" in action:
```
head → item(5) → item(8) → item(3) → tail
```

```graphoid
# Queue - A Graph-Based FIFO Data Structure
# Demonstrates: Data structures as graphs, not just state machines

graph Queue {
    head_id: none
    tail_id: none
    size: 0

    fn new() {
        instance = self.clone()
        instance.head_id = none
        instance.tail_id = none
        instance.size = 0
        return instance
    }

    get length() { return size }
    get is_empty() { return size == 0 }

    fn enqueue(value) {
        # Create node for this item
        node_id = "node_" + size.to_string() + "_" + value.to_string()
        self.add_node(node_id, {"value": value})

        if is_empty {
            head_id = node_id
            tail_id = node_id
        } else {
            # Link from current tail to new node
            self.add_edge(tail_id, node_id, "next")
            tail_id = node_id
        }

        size = size + 1
        return self
    }

    fn dequeue() {
        if is_empty { return none }

        # Get value from head
        head_data = self.node_data(head_id)
        value = head_data["value"]
        old_head = head_id

        # Find next node (if any)
        next_id = find_next(head_id)

        if next_id == none {
            # Queue is now empty
            head_id = none
            tail_id = none
        } else {
            head_id = next_id
        }

        # Remove old head node
        self.remove_node(old_head)
        size = size - 1

        return value
    }

    fn peek() {
        if is_empty { return none }
        head_data = self.node_data(head_id)
        return head_data["value"]
    }

    fn contains(value) {
        for node in self.nodes() {
            data = self.node_data(node)
            if data["value"] == value { return true }
        }
        return false
    }

    fn to_list() {
        result = []
        current = head_id
        while current != none {
            data = self.node_data(current)
            result = result.append(data["value"])
            current = find_next(current)
        }
        return result
    }

    priv fn find_next(node_id) {
        for edge in self.edges() {
            if edge[0] == node_id && edge[2] == "next" {
                return edge[1]
            }
        }
        return none
    }
}
```

---

#### 2B-2. CallBox CLG (`samples/06-projects/elevator/call_box.gr`)

**Emergency communication system** - another state machine:

```graphoid
# CallBox - Emergency Phone State Machine
# Demonstrates: Small focused CLG, composition

graph CallBox {
    current_state: "idle"
    call_duration: 0
    dispatcher_message: ""

    fn new() {
        instance = self.clone()
        instance.current_state = "idle"
        instance.call_duration = 0
        instance.dispatcher_message = ""

        # State machine structure
        instance.add_node("idle", {"type": "state"})
        instance.add_node("connecting", {"type": "state"})
        instance.add_node("connected", {"type": "state"})
        instance.add_node("on_hold", {"type": "state"})

        instance.add_edge("idle", "connecting", "pick_up")
        instance.add_edge("connecting", "connected", "dispatcher_answers")
        instance.add_edge("connecting", "idle", "timeout")
        instance.add_edge("connected", "idle", "hang_up")
        instance.add_edge("connected", "on_hold", "hold")
        instance.add_edge("on_hold", "connected", "resume")
        instance.add_edge("on_hold", "idle", "hang_up")

        return instance
    }

    get state() { return current_state }
    get is_active() { return current_state != "idle" }

    fn pick_up() {
        if current_state == "idle" {
            current_state = "connecting"
            print("    [CallBox] Connecting to emergency dispatch...")
            return true
        }
        return false
    }

    fn dispatcher_answers(message) {
        if current_state == "connecting" {
            current_state = "connected"
            dispatcher_message = message
            print("    [CallBox] Dispatcher: " + message)
            return true
        }
        return false
    }

    fn hang_up() {
        if current_state != "idle" {
            current_state = "idle"
            dispatcher_message = ""
            call_duration = 0
            print("    [CallBox] Call ended")
            return true
        }
        return false
    }

    fn speak(message) {
        if current_state == "connected" {
            print("    [CallBox] Passenger: " + message)
            return true
        }
        return false
    }
}
```

---

#### 2B-3. Elevator CLG (`samples/06-projects/elevator/elevator.gr`)

**Main elevator** - imports and composes Queue and CallBox:

```graphoid
# Elevator - Main Controller
# Demonstrates: CLG composition, multiple imports, complex state machine

import "queue"
import "call_box"

graph Elevator {
    current_floor: 1
    current_state: "idle"
    direction: "none"
    num_floors: 10
    door_obstruction_count: 0
    request_queue: none       # Queue CLG instance
    phone: none               # CallBox CLG instance

    fn new(floors) {
        instance = self.clone()
        instance.num_floors = floors
        instance.current_floor = 1
        instance.current_state = "idle"
        instance.direction = "none"
        instance.door_obstruction_count = 0

        # Compose with other CLGs
        instance.request_queue = Queue.new()
        instance.phone = CallBox.new()

        # Build state machine structure
        # Normal states
        instance.add_node("idle", {"type": "normal"})
        instance.add_node("moving_up", {"type": "normal"})
        instance.add_node("moving_down", {"type": "normal"})
        instance.add_node("doors_opening", {"type": "normal"})
        instance.add_node("doors_open", {"type": "normal"})
        instance.add_node("doors_closing", {"type": "normal"})

        # Emergency states
        instance.add_node("emergency_stop", {"type": "emergency"})
        instance.add_node("fire_mode", {"type": "emergency"})
        instance.add_node("power_failure", {"type": "emergency"})
        instance.add_node("door_obstruction", {"type": "emergency"})

        # Normal transitions
        instance.add_edge("idle", "moving_up", "go_up")
        instance.add_edge("idle", "moving_down", "go_down")
        instance.add_edge("idle", "doors_opening", "open_doors")
        instance.add_edge("moving_up", "doors_opening", "arrive")
        instance.add_edge("moving_down", "doors_opening", "arrive")
        instance.add_edge("doors_opening", "doors_open", "opened")
        instance.add_edge("doors_open", "doors_closing", "close_doors")
        instance.add_edge("doors_closing", "idle", "closed")
        instance.add_edge("doors_closing", "door_obstruction", "obstruction")

        # Emergency transitions (can happen from most states)
        instance.add_edge("idle", "emergency_stop", "emergency")
        instance.add_edge("moving_up", "emergency_stop", "emergency")
        instance.add_edge("moving_down", "emergency_stop", "emergency")
        instance.add_edge("doors_open", "emergency_stop", "emergency")

        instance.add_edge("idle", "fire_mode", "fire_alarm")
        instance.add_edge("moving_up", "fire_mode", "fire_alarm")
        instance.add_edge("moving_down", "fire_mode", "fire_alarm")
        instance.add_edge("doors_open", "fire_mode", "fire_alarm")

        instance.add_edge("moving_up", "power_failure", "power_lost")
        instance.add_edge("moving_down", "power_failure", "power_lost")

        # Recovery transitions
        instance.add_edge("emergency_stop", "idle", "reset")
        instance.add_edge("door_obstruction", "doors_opening", "clear")
        instance.add_edge("door_obstruction", "emergency_stop", "max_retries")
        instance.add_edge("power_failure", "idle", "power_restored")

        return instance
    }

    # Computed properties
    get floor() { return current_floor }
    get state() { return current_state }
    get pending_requests() { return request_queue.length }
    get is_idle() { return current_state == "idle" && pending_requests == 0 }
    get is_emergency() {
        state_data = self.node_data(current_state)
        return state_data["type"] == "emergency"
    }

    # Request a floor
    fn request(floor) {
        if is_emergency {
            print("  Cannot accept requests during emergency")
            return self
        }

        if floor < 1 || floor > num_floors {
            print("  Invalid floor: " + floor.to_string())
            return self
        }

        if !request_queue.contains(floor) {
            request_queue.enqueue(floor)
            print("  Request queued: floor " + floor.to_string())
        }

        return self
    }

    # Process one step
    fn step() {
        if is_emergency {
            handle_emergency_step()
            return self
        }

        if current_state == "idle" {
            if pending_requests == 0 {
                return self
            }

            next_floor = choose_next_floor()

            if next_floor > current_floor {
                direction = "up"
                current_state = "moving_up"
                print("  Moving up...")
            } else if next_floor < current_floor {
                direction = "down"
                current_state = "moving_down"
                print("  Moving down...")
            } else {
                current_state = "doors_opening"
                print("  Opening doors at floor " + current_floor.to_string())
            }

        } else if current_state == "moving_up" {
            current_floor = current_floor + 1
            print("  Floor " + current_floor.to_string())

            if should_stop() {
                current_state = "doors_opening"
                print("  Stopping - opening doors")
            }

        } else if current_state == "moving_down" {
            current_floor = current_floor - 1
            print("  Floor " + current_floor.to_string())

            if should_stop() {
                current_state = "doors_opening"
                print("  Stopping - opening doors")
            }

        } else if current_state == "doors_opening" {
            current_state = "doors_open"
            remove_current_floor_request()
            print("  Doors open")

        } else if current_state == "doors_open" {
            current_state = "doors_closing"
            print("  Doors closing...")

        } else if current_state == "doors_closing" {
            current_state = "idle"
            door_obstruction_count = 0
            print("  Doors closed")

        } else if current_state == "door_obstruction" {
            door_obstruction_count = door_obstruction_count + 1
            if door_obstruction_count >= 3 {
                current_state = "emergency_stop"
                print("  Max retries - emergency stop!")
                phone.pick_up()
            } else {
                current_state = "doors_opening"
                print("  Obstruction cleared, retry " + door_obstruction_count.to_string())
            }
        }

        return self
    }

    # Emergency triggers
    fn trigger_emergency() {
        print("  !!! EMERGENCY STOP !!!")
        current_state = "emergency_stop"
        phone.pick_up()
        return self
    }

    fn trigger_fire_alarm() {
        print("  !!! FIRE ALARM - RETURNING TO GROUND FLOOR !!!")
        current_state = "fire_mode"
        # Clear all requests
        while !request_queue.is_empty {
            request_queue.dequeue()
        }
        return self
    }

    fn trigger_obstruction() {
        if current_state == "doors_closing" {
            print("  Obstruction detected!")
            current_state = "door_obstruction"
        }
        return self
    }

    fn reset() {
        if current_state == "emergency_stop" {
            print("  System reset")
            current_state = "idle"
            phone.hang_up()
        }
        return self
    }

    # Run until idle or emergency
    fn run() {
        step_count = 0
        max_steps = 100

        while !is_idle && !is_emergency && step_count < max_steps {
            step()
            step_count = step_count + 1
        }

        return self
    }

    fn status() {
        emergency_flag = ""
        if is_emergency { emergency_flag = " [EMERGENCY]" }

        print("Floor: " + current_floor.to_string() +
              " | State: " + current_state +
              " | Queue: " + request_queue.to_list().to_string() +
              emergency_flag)
    }

    fn use_phone() {
        return phone
    }

    # Private helpers
    priv fn choose_next_floor() {
        if request_queue.is_empty { return current_floor }

        # SCAN algorithm: continue in current direction if requests exist
        queue_list = request_queue.to_list()

        if direction == "up" {
            # Find next floor above
            for floor in queue_list {
                if floor > current_floor { return floor }
            }
        } else if direction == "down" {
            # Find next floor below
            for floor in queue_list {
                if floor < current_floor { return floor }
            }
        }

        # No requests in current direction, pick nearest
        return queue_list[0]
    }

    priv fn should_stop() {
        return request_queue.contains(current_floor)
    }

    priv fn remove_current_floor_request() {
        # Rebuild queue without current floor
        temp_list = []
        while !request_queue.is_empty {
            floor = request_queue.dequeue()
            if floor != current_floor {
                temp_list = temp_list.append(floor)
            }
        }
        for floor in temp_list {
            request_queue.enqueue(floor)
        }
    }

    priv fn handle_emergency_step() {
        if current_state == "fire_mode" {
            if current_floor > 1 {
                current_floor = current_floor - 1
                print("  [FIRE MODE] Descending... floor " + current_floor.to_string())
            } else {
                print("  [FIRE MODE] Ground floor reached. Doors open. Elevator disabled.")
            }
        } else if current_state == "power_failure" {
            print("  [POWER FAILURE] On battery. Call for help.")
            if !phone.is_active {
                phone.pick_up()
            }
        }
    }
}
```

---

#### 2B-4. Interactive Demo (`samples/06-projects/elevator/main.gr`)

```graphoid
# Elevator Simulator - Interactive Demo
# Run: graphoid samples/06-projects/elevator/main.gr

import "elevator"
import "queue"
import "io"

# Create elevator
elevator = Elevator.new(10)

fn show_help() {
    print("")
    print("Commands:")
    print("  request N    - Request floor N (1-10)")
    print("  step         - Advance one simulation step")
    print("  run          - Run until idle or emergency")
    print("  status       - Show current elevator state")
    print("  fire         - Trigger fire alarm")
    print("  emergency    - Trigger emergency stop")
    print("  obstruction  - Simulate door obstruction")
    print("  reset        - Reset from emergency state")
    print("  phone        - Use emergency phone")
    print("  graph        - Show state machine structure")
    print("  queue        - Show request queue as graph")
    print("  help         - Show this help")
    print("  quit         - Exit simulator")
    print("")
}

fn show_graph() {
    print("")
    print("=== State Machine Structure ===")
    print("Normal states: idle, moving_up, moving_down, doors_opening, doors_open, doors_closing")
    print("Emergency states: emergency_stop, fire_mode, power_failure, door_obstruction")
    print("")
    print("Transitions:")
    for state in ["idle", "moving_up", "moving_down", "doors_open", "emergency_stop"] {
        transitions = []
        for edge in elevator.edges() {
            if edge[0] == state {
                transitions = transitions.append(edge[2] + " -> " + edge[1])
            }
        }
        if transitions.length() > 0 {
            print("  " + state + ": " + transitions.to_string())
        }
    }
    print("")
}

fn show_queue_graph() {
    print("")
    print("=== Request Queue (Graph Structure) ===")
    q = elevator.request_queue
    print("Queue contents: " + q.to_list().to_string())
    print("Graph nodes: " + q.node_count().to_string())
    print("Graph edges: " + q.edge_count().to_string())
    if !q.is_empty {
        print("Structure: head")
        current = q.head_id
        while current != none {
            data = q.node_data(current)
            next = none
            for edge in q.edges() {
                if edge[0] == current && edge[2] == "next" {
                    next = edge[1]
                }
            }
            if next != none {
                print("  -> " + data["value"].to_string())
            } else {
                print("  -> " + data["value"].to_string() + " (tail)")
            }
            current = next
        }
    }
    print("")
}

fn use_phone() {
    phone = elevator.use_phone()
    if !phone.is_active {
        print("Phone is not active (only available during emergencies)")
        return
    }
    print("")
    print("=== Emergency Phone ===")
    phone.dispatcher_answers("Emergency services. What is your location?")
    print("(Passenger speaks into phone)")
    phone.speak("I'm stuck in elevator at floor " + elevator.floor.to_string())
    print("")
}

# Main loop
print("=== Elevator Simulator ===")
print("A 10-floor elevator demonstrating Class-Like Graphs")
print("Type 'help' for commands.")
print("")
elevator.status()

running = true
while running {
    print("")
    input = io.prompt("> ")

    if input == "quit" || input == "exit" {
        print("Goodbye!")
        running = false
    } else if input == "help" {
        show_help()
    } else if input == "status" {
        elevator.status()
    } else if input == "step" {
        elevator.step()
        elevator.status()
    } else if input == "run" {
        elevator.run()
        elevator.status()
    } else if input == "fire" {
        elevator.trigger_fire_alarm()
    } else if input == "emergency" {
        elevator.trigger_emergency()
    } else if input == "obstruction" {
        elevator.trigger_obstruction()
    } else if input == "reset" {
        elevator.reset()
        elevator.status()
    } else if input == "phone" {
        use_phone()
    } else if input == "graph" {
        show_graph()
    } else if input == "queue" {
        show_queue_graph()
    } else if input.starts_with("request ") {
        floor_str = input.substring(8)
        floor = floor_str.to_num()
        if floor != none {
            elevator.request(floor)
        } else {
            print("Invalid floor number")
        }
    } else if input == "" {
        # Empty input, just continue
    } else {
        print("Unknown command. Type 'help' for available commands.")
    }
}
```

### 2C. Social Graph Analyzer (`samples/06-projects/social/`)

**Description:** Model and analyze social networks using CLGs.

**File structure:**
```
samples/06-projects/social/
├── README.md              # Project documentation
├── social_network.gr      # SocialNetwork CLG
└── main.gr                # Demo script
```

**Full implementation:**
```graphoid
# Social Graph Analyzer - Network Analysis with CLGs
# Demonstrates: CLG for network analysis, semantic edges, computed properties

graph SocialNetwork {
    name: "Social Network"

    fn new(network_name) {
        instance = self.clone()
        instance.name = network_name
        return instance
    }

    fn add_user(username, profile) {
        self.add_node(username, profile)
        return self
    }

    fn add_friendship(user1, user2) {
        # Friendships are bidirectional
        self.add_edge(user1, user2, "friend")
        self.add_edge(user2, user1, "friend")
        return self
    }

    fn add_follow(follower, followed) {
        # Follows are unidirectional
        self.add_edge(follower, followed, "follows")
        return self
    }

    # Computed properties
    get user_count() { return self.node_count() }
    get connection_count() { return self.edge_count() / 2 }  # Friendships counted twice

    # Analysis methods
    fn friends_of(username) {
        result = []
        for edge in self.edges() {
            if edge[0] == username && edge[2] == "friend" {
                result = result.append(edge[1])
            }
        }
        return result
    }

    fn mutual_friends(user1, user2) {
        friends1 = friends_of(user1)
        friends2 = friends_of(user2)

        result = []
        for f in friends1 {
            for f2 in friends2 {
                if f == f2 {
                    result = result.append(f)
                }
            }
        }
        return result
    }

    fn friend_count(username) {
        return friends_of(username).length()
    }

    fn degrees_of_separation(user1, user2) {
        path = self.shortest_path(user1, user2)
        if path == none {
            return -1  # Not connected
        }
        return path.length() - 1
    }

    fn most_connected() {
        max_friends = 0
        top_user = none

        for node in self.nodes() {
            count = friend_count(node)
            if count > max_friends {
                max_friends = count
                top_user = node
            }
        }

        return {"user": top_user, "friends": max_friends}
    }

    fn network_report() {
        print("=== " + name + " Report ===")
        print("Users: " + user_count.to_string())
        print("Connections: " + connection_count.to_string())
        print("")

        most = most_connected()
        print("Most connected: " + most["user"] + " with " + most["friends"].to_string() + " friends")

        print("")
        print("Semantic edge analysis:")
        print("Methods that read nodes: " + self.property_readers("nodes").to_string())
    }
}

# === Demo ===
print("=== Social Network Demo ===")
print("")

network = SocialNetwork.new("Tech Community")

# Add users
network.add_user("alice", {"role": "engineer", "company": "TechCorp"})
network.add_user("bob", {"role": "designer", "company": "DesignCo"})
network.add_user("charlie", {"role": "manager", "company": "TechCorp"})
network.add_user("diana", {"role": "engineer", "company": "StartupX"})
network.add_user("eve", {"role": "analyst", "company": "DataCo"})

# Add friendships
network.add_friendship("alice", "bob")
network.add_friendship("alice", "charlie")
network.add_friendship("alice", "diana")
network.add_friendship("bob", "charlie")
network.add_friendship("charlie", "eve")
network.add_friendship("diana", "eve")

# Generate report
network.network_report()

print("")
print("=== Analysis ===")
print("Alice's friends: " + network.friends_of("alice").to_string())
print("Bob's friends: " + network.friends_of("bob").to_string())
print("Mutual friends of Alice & Bob: " + network.mutual_friends("alice", "bob").to_string())
print("Degrees between Bob and Diana: " + network.degrees_of_separation("bob", "diana").to_string())

# Visualize
print("")
print("=== Network Visualization ===")
print(network.to_dot())
```

---

## Part 3: Documentation Updates

### 3A. New User Guide Chapter

**File:** `docs/user-guide/11-class-like-graphs.md`

**Outline:**
1. Introduction - What are CLGs?
2. Declaring CLGs (`graph Name { }`)
3. Properties and implicit self
4. Computed properties (`get`)
5. Private methods (`priv`)
6. Inheritance (`from`)
7. Static methods
8. Setters
9. Type checking (`is_a()`, `responds_to()`)
10. Mixins (`include()`)
11. **Semantic Edges** - Key differentiator!
    - Method-property edges (`method_reads`, `method_writes`)
    - Dependency tracking (`dependencies`, `dependents`)
    - Inheritance as graph structure (`ancestors`)
12. Best practices

### 3B. API Reference

**File:** `docs/api-reference/clg.md`

**Contents:**
- Declaration syntax
- Property access
- Computed properties
- Private methods
- Inheritance
- Type checking methods
- All semantic edge methods with examples

### 3C. Update Existing Docs

- `docs/user-guide/06-graph-operations.md` - Add link to CLG chapter
- `docs/DESIGN_PHILOSOPHY.md` - Add CLG as "everything is a graph" example

---

## Part 4: Gap Analysis

### Completed Features
- All CLG features (Phases 1-23)
- Semantic edges (method-property, dependencies, inheritance)
- Error message enhancements (property suggestions)

### Low-Priority Gaps (Not Blocking)
| Gap | Priority | Notes |
|-----|----------|-------|
| Abstract methods | Low | Use duck typing instead |
| Interfaces/protocols | Low | `responds_to()` provides this |
| Private visibility in viz | Medium | Nice to have |
| Clone with mods | Medium | `obj.clone({ prop: val })` |

### Intentional Non-Features
- No multiple inheritance (use mixins)
- No generics (runtime type checks)
- No abstract base classes (duck typing)

---

## Implementation Order

### Phase A: Stdlib CLGs
1. ✅ Create `stdlib/dataset.gr` - DONE (December 16, 2025)
2. Create `stdlib/datetime.gr` - DEFERRED (lower priority)
3. ✅ Test DataSet - DONE

### Phase B: Sample Projects
1. ✅ Create `samples/06-projects/` with per-project subdirectories - DONE
2. ✅ Climate project (`samples/06-projects/climate/`) - DONE (December 16, 2025)
   - `README.md`, `data/global_temperature.json`
   - `climate_analyzer.gr` (CLG using DataSet)
   - `main.gr` (demo)
3. **NEXT:** Elevator project (`samples/06-projects/elevator/`):
   - `README.md`
   - `queue.gr` - graph-based FIFO CLG
   - `call_box.gr` - emergency phone CLG
   - `elevator.gr` - main controller (imports Queue, CallBox)
   - `main.gr` - **interactive** demo
4. Social project (`samples/06-projects/social/`):
   - `README.md`
   - `social_network.gr` (CLG)
   - `main.gr` (demo)

### Phase C: Documentation
1. Write user guide chapter
2. Write API reference
3. Update existing docs

### Phase D: Review
1. Run all samples
2. Update gap doc

---

## Files Summary

**Created (December 16, 2025):**
- ✅ `stdlib/dataset.gr`
- ✅ `samples/06-projects/climate/README.md`
- ✅ `samples/06-projects/climate/data/global_temperature.json` (NASA GISTEMP, public domain)
- ✅ `samples/06-projects/climate/climate_analyzer.gr`
- ✅ `samples/06-projects/climate/main.gr`

**Modified (December 16, 2025):**
- ✅ `src/execution/executor.rs` - Bug fix for getter access in private methods

**Still To Create:**
- `stdlib/datetime.gr` (deferred - lower priority)

- `samples/06-projects/elevator/README.md`
- `samples/06-projects/elevator/queue.gr` (graph-based FIFO)
- `samples/06-projects/elevator/call_box.gr` (emergency phone CLG)
- `samples/06-projects/elevator/elevator.gr` (imports Queue, CallBox)
- `samples/06-projects/elevator/main.gr` (interactive demo)

- `samples/06-projects/social/README.md`
- `samples/06-projects/social/social_network.gr`
- `samples/06-projects/social/main.gr`

- `docs/user-guide/11-class-like-graphs.md`
- `docs/api-reference/clg.md`

**Still To Modify:**
- `docs/user-guide/06-graph-operations.md`
- `docs/DESIGN_PHILOSOPHY.md`
- `dev_docs/CLASS_LIKE_GRAPHS_GAPS.md`

---

## Progress (December 16, 2025)

### Completed
- [x] `stdlib/dataset.gr` - DataSet CLG with full statistical analysis
- [x] `samples/06-projects/climate/` - Climate Analyzer project complete
  - README.md, climate_analyzer.gr, main.gr
  - Uses NASA GISTEMP data (1880-2023)
  - Demonstrates CLG composition with DataSet

### Bug Fixed
- **Private methods couldn't access computed properties (getters)**
- Fix in `src/execution/executor.rs` lines 209-220
- Added getter check in implicit self resolution

### Issues Discovered
| Issue | Solution |
|-------|----------|
| `list` is reserved word | Use different parameter name like `sorted_list` |
| `!var` not supported | Use `var == false` |
| CLG not in scope after import | Must use module namespace: `module.CLGName` |
| Private method self-calls | Use `_methodname()` not `self._methodname()` |
| `io.read` doesn't exist | Use `io.read_file` |
| Method chaining doesn't persist | Requires reassignment: `obj = obj.method()` |

### Next Up
- [ ] **Elevator project** - Queue CLG, CallBox CLG, Elevator CLG, interactive demo
- [ ] **Social network project** - SocialNetwork CLG
- [ ] **Documentation** - User guide chapter, API reference

---

## Notes for Implementation

1. **Module declaration required** - Every CLG file needs `module name_module alias name`
2. **Elevator is the flagship** - 3 CLGs composing together, interactive mode, emergencies
3. **Queue CLG demonstrates "everything is a graph"** - Data structure as graph nodes/edges
4. **Test count: 1187** (1174 unit + 13 doc tests) - System is stable
5. **Run climate project to verify**:
   ```bash
   ~/.cargo/bin/cargo run --quiet samples/06-projects/climate/main.gr
   ```
