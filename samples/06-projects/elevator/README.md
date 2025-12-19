# Elevator Simulator

A multi-elevator building simulator demonstrating Class-Like Graphs (CLGs) and the "everything is a graph" philosophy.

## Status

**Current:** Complete multi-elevator system (December 2025)

- [x] Single elevator with state machine
- [x] Multiple elevators (up to 4) with intelligent dispatch
- [x] Button panel for hall calls
- [x] Interactive REPL-style controller
- [x] Virtual passenger simulation for stress testing

## Run

```bash
# Single elevator demo
graphoid samples/06-projects/elevator/main.gr

# Multi-elevator demo (3 elevators with dispatch algorithm)
graphoid samples/06-projects/elevator/multi_main.gr

# Interactive controller (REPL-style)
graphoid samples/06-projects/elevator/interactive.gr

# Virtual passenger simulation (stress test)
graphoid samples/06-projects/elevator/sim_demo.gr
```

## Interactive Commands

The interactive controller supports these commands:

| Command | Description |
|---------|-------------|
| `call <floor> <up\|down>` | Press hall button on a floor |
| `go <elevator> <floor>` | From inside elevator, select floor |
| `step [n]` | Advance n time ticks (default 1) |
| `run` | Run until all elevators idle |
| `status` | Show all elevator status |
| `fire` | Trigger building fire alarm |
| `reset` | Reset all elevators from emergency |
| `help` | Show available commands |
| `quit` | Exit the controller |

## What This Demonstrates

### 1. CLG Composition
Seven CLGs work together in a hierarchy:
- **Simulation** - Virtual passenger stress testing (contains Building, Passengers)
- **Building** - Multi-elevator controller (contains Elevators)
- **Elevator** - Single elevator controller (contains Queue and CallBox)
- **Queue** - Graph-based FIFO request queue
- **CallBox** - Emergency phone state machine
- **ButtonPanel** - Hall call buttons (tracks button state as graph nodes)
- **Passenger** - Virtual person with wait/ride time tracking

### 2. "Everything is a Graph"

**Queue as a Graph:**
The request queue is not a wrapper around a list - it's implemented as actual graph nodes connected by "next" edges:
```
node_1(floor:5) --next--> node_2(floor:8) --next--> node_3(floor:3)
```

**State Machine as a Graph:**
Elevator states are graph nodes, transitions are edges:
```
idle --go_up--> moving_up --arrive--> doors_opening
     --fire_alarm--> fire_mode
```

### 3. State Machine with Emergency Modes

**Normal States:**
- `idle` - Waiting for requests
- `moving_up` / `moving_down` - Traveling between floors
- `doors_opening` / `doors_open` / `doors_closing` - Door operations

**Emergency States:**
- `emergency_stop` - Manual emergency stop, activates phone
- `fire_mode` - Returns to ground floor, clears all requests
- `door_obstruction` - Retries door close, escalates to emergency after 3 attempts

## File Structure

```
elevator/
├── README.md         # This file
├── queue.gr          # Graph-based FIFO queue CLG
├── call_box.gr       # Emergency phone state machine CLG
├── elevator.gr       # Single elevator controller CLG
├── building.gr       # Multi-elevator building controller CLG
├── panel.gr          # Hall call button panel CLG
├── passenger.gr      # Virtual passenger CLG
├── simulation.gr     # Passenger simulation CLG
├── main.gr           # Single elevator demo
├── multi_main.gr     # Multi-elevator demo
├── interactive.gr    # Interactive REPL controller
└── sim_demo.gr       # Simulation stress test demo
```

## Dispatch Algorithm

The Building CLG uses an intelligent dispatch algorithm to assign the best elevator to each hall call:

1. **Prefer idle elevators** closest to the calling floor
2. **Prefer elevators moving toward** the call in the same direction
3. **Penalize elevators moving away** from the call
4. **Skip elevators in emergency state**

Score calculation (lower = better):
- Base score = distance to calling floor
- Idle elevator = base score only
- Moving toward call = base score
- Moving toward but wrong direction = base score + 5
- Moving away = base score + 20
- Door operation = base score + 3

## CLG Features Used

| Feature | Where Used |
|---------|------------|
| Graph declaration | All seven CLGs |
| Properties | `_floor`, `_state`, `_waiting`, `_completed`, etc. |
| Methods | `request()`, `step()`, `call()`, `tick()`, `report()`, etc. |
| Private methods | `_dispatch()`, `_do_boarding()`, `_do_arrivals()` |
| CLG composition | Simulation > Building > Elevator > Queue/CallBox |
| Graph operations | `add_node()`, `add_edge()`, `edges()`, `get_node()` |
| Iteration | `for p in _waiting`, `for elevator_id in _elevators` |
| Statistics | Wait time, ride time, avg/max calculations |

## Sample Output

```
=== Elevator Simulator ===
A 10-floor elevator demonstrating Class-Like Graphs

--- Demo 1: Basic Operation ---

  Floor: 1 | State: idle | Queue: []

Requesting floors 5 and 8...
  Request queued: floor 5
  Request queued: floor 8

Running elevator to completion...
  Moving up...
  Floor 2
  Floor 3
  Floor 4
  Floor 5
  Stopping - opening doors
  Doors open
  Doors closing...
  Doors closed
  Moving up...
  Floor 6
  Floor 7
  Floor 8
  Stopping - opening doors
  Doors open
  Doors closing...
  Doors closed

  Floor: 8 | State: idle | Queue: []
```

## Simulation Model: Tick-Based (Not Async)

This simulator uses a **discrete tick-based model** rather than real-time concurrent processes. Each call to `step()` or `tick()` advances the simulation by one time unit.

**Why tick-based?**

Graphoid does not yet have async/await or concurrency primitives (planned for Phase 15). The tick-based approach:

- Works within current language capabilities
- Makes behavior deterministic and testable
- Allows precise control over simulation timing
- Is actually how many real elevator controllers work internally

**How it works:**

```graphoid
# Each tick, all elevators advance one step
building.step()  # Moves each elevator, opens/closes doors, etc.

# Or run until all elevators are idle
building.run()   # Calls step() repeatedly until done
```

**Future enhancement:** Once Graphoid has async support, elevators could run as independent actors with message passing, more closely modeling real concurrent systems.

## Key Takeaways

1. **Data structures ARE graphs** - The Queue CLG stores items as nodes with "next" edges
2. **State machines ARE graphs** - States are nodes, transitions are edges
3. **CLGs compose naturally** - Elevator uses Queue and CallBox as components
4. **Emergency handling** - Fire mode and emergency stop demonstrate complex state transitions
5. **Tick-based simulation** - Works without async; each `step()` advances all elevators
