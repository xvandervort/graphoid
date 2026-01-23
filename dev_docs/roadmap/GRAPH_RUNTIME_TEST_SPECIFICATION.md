# Graph Runtime Test & Benchmark Specification

**Version**: 1.0
**Created**: January 15, 2026
**Purpose**: Define tests and benchmarks for evaluating graph-centric runtime implementations

---

## Overview

Before implementing any graph-centric runtime experiments, we need:

1. **Correctness Tests** - Ensure graph implementations produce identical results
2. **Performance Benchmarks** - Measure overhead of graph operations
3. **Memory Benchmarks** - Measure space overhead
4. **Stress Tests** - Verify behavior under extreme conditions
5. **Graph-Specific Tests** - Validate graph-centric capabilities

All experiments must pass correctness tests and meet performance targets.

---

## Part 1: Correctness Tests

### 1.1 Semantic Equivalence

Every graph-based implementation must produce **identical results** to the current implementation for all existing tests.

```bash
# Run all existing tests against experimental implementation
GRAPHOID_RUNTIME=graph cargo test --lib
GRAPHOID_RUNTIME=graph gr spec tests/gspec/
```

**Criteria**: 100% of existing tests must pass. Zero regressions.

### 1.2 Variable Binding Correctness

```graphoid
# test_binding_basic.gr
x = 5
assert(x == 5, "basic binding")

# test_binding_reassign.gr
x = 5
x = 10
assert(x == 10, "reassignment")

# test_binding_shadow.gr
x = 5
fn inner() {
    x = 10  # shadows outer
    assert(x == 10, "inner shadow")
}
inner()
assert(x == 5, "outer unchanged")

# test_binding_closure.gr
x = 5
fn make_adder() {
    return fn(y) { return x + y }
}
add5 = make_adder()
assert(add5(3) == 8, "closure capture")

# test_binding_deep_scope.gr
a = 1
fn level1() {
    b = 2
    fn level2() {
        c = 3
        fn level3() {
            d = 4
            fn level4() {
                e = 5
                return a + b + c + d + e
            }
            return level4()
        }
        return level3()
    }
    return level2()
}
assert(level1() == 15, "deep scope access")
```

### 1.3 Execution Correctness

```graphoid
# test_exec_arithmetic.gr
assert(2 + 3 * 4 == 14, "precedence")
assert((2 + 3) * 4 == 20, "grouping")
assert(-5 + 10 == 5, "unary minus")
assert(10 / 3 == 3.333..., "division")  # within epsilon
assert(10 % 3 == 1, "modulo")

# test_exec_comparison.gr
assert((5 > 3) == true, "greater")
assert((5 < 3) == false, "less")
assert((5 == 5) == true, "equal")
assert((5 != 3) == true, "not equal")

# test_exec_logical.gr
assert((true and false) == false, "and")
assert((true or false) == true, "or")
assert(not true == false, "not")

# test_exec_control_flow.gr
x = 0
if true { x = 1 }
assert(x == 1, "if true")

x = 0
if false { x = 1 } else { x = 2 }
assert(x == 2, "if-else")

x = 0
while x < 5 { x = x + 1 }
assert(x == 5, "while loop")

total = 0
for i in [1, 2, 3, 4, 5] { total = total + i }
assert(total == 15, "for loop")

# test_exec_functions.gr
fn add(a, b) { return a + b }
assert(add(3, 4) == 7, "function call")

fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}
assert(factorial(5) == 120, "recursion")

# test_exec_pattern_match.gr
fn describe(x) {
    match x {
        0 => "zero"
        n if n > 0 => "positive"
        _ => "negative"
    }
}
assert(describe(0) == "zero", "match literal")
assert(describe(5) == "positive", "match guard")
assert(describe(-3) == "negative", "match wildcard")
```

### 1.4 Collection Correctness

```graphoid
# test_collections_list.gr
lst = [1, 2, 3]
assert(lst.length() == 3, "list length")
assert(lst[0] == 1, "list index")
lst.append(4)
assert(lst.length() == 4, "list append")
assert(lst.map(x => x * 2) == [2, 4, 6, 8], "list map")
assert(lst.filter(x => x > 2) == [3, 4], "list filter")

# test_collections_map.gr
m = {"a": 1, "b": 2}
assert(m["a"] == 1, "map access")
m["c"] = 3
assert(m.keys().length() == 3, "map keys")
assert(m.values().sum() == 6, "map values")

# test_collections_graph.gr
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_edge("A", "B", "connects")
assert(g.nodes().length() == 2, "graph nodes")
assert(g.edges().length() == 1, "graph edges")
assert(g.neighbors("A") == ["B"], "graph neighbors")
```

### 1.5 Module Correctness

```graphoid
# test_modules.gr
import "math"
assert(math.abs(-5) == 5, "stdlib import")
assert(math.sqrt(16) == 4, "stdlib function")

# test_module_local.gr (with local module)
import "my_module"
assert(my_module.greet("world") == "hello world", "local module")
```

---

## Part 2: Performance Benchmarks

### 2.1 Benchmark Infrastructure

```rust
// benchmarks/infrastructure.rs

use std::time::{Duration, Instant};

pub struct BenchmarkResult {
    pub name: String,
    pub iterations: u64,
    pub total_time: Duration,
    pub mean_time: Duration,
    pub std_dev: Duration,
    pub ops_per_second: f64,
}

pub fn benchmark<F>(name: &str, iterations: u64, mut f: F) -> BenchmarkResult
where F: FnMut()
{
    // Warmup
    for _ in 0..100 {
        f();
    }

    // Collect samples
    let mut samples = Vec::with_capacity(iterations as usize);
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        samples.push(start.elapsed());
    }

    // Calculate statistics
    let total: Duration = samples.iter().sum();
    let mean = total / iterations as u32;
    let variance: f64 = samples.iter()
        .map(|d| {
            let diff = d.as_nanos() as f64 - mean.as_nanos() as f64;
            diff * diff
        })
        .sum::<f64>() / iterations as f64;
    let std_dev = Duration::from_nanos(variance.sqrt() as u64);

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_time: total,
        mean_time: mean,
        std_dev,
        ops_per_second: 1_000_000_000.0 / mean.as_nanos() as f64,
    }
}
```

### 2.2 Variable Lookup Benchmarks

```graphoid
# bench_lookup_local.gr
# Measure: local variable lookup
fn bench_local(n) {
    x = 42
    total = 0
    i = 0
    while i < n {
        total = total + x  # local lookup
        i = i + 1
    }
    return total
}
# Run: bench_local(1_000_000)
# Measure: time per iteration

# bench_lookup_nested.gr
# Measure: nested scope lookup (closure-like)
fn bench_nested(n) {
    x = 42
    fn inner() {
        total = 0
        i = 0
        while i < n {
            total = total + x  # outer scope lookup
            i = i + 1
        }
        return total
    }
    return inner()
}
# Run: bench_nested(1_000_000)

# bench_lookup_deep.gr
# Measure: deep scope chain (worst case)
fn bench_deep(n) {
    a = 1
    fn l1() {
        b = 2
        fn l2() {
            c = 3
            fn l3() {
                d = 4
                fn l4() {
                    e = 5
                    fn l5() {
                        total = 0
                        i = 0
                        while i < n {
                            total = total + a  # 5 levels up
                            i = i + 1
                        }
                        return total
                    }
                    return l5()
                }
                return l4()
            }
            return l3()
        }
        return l2()
    }
    return l1()
}
# Run: bench_deep(100_000)

# bench_lookup_many_vars.gr
# Measure: lookup in scope with many variables
fn bench_many_vars(n) {
    # Create 100 variables
    v001 = 1; v002 = 2; v003 = 3; v004 = 4; v005 = 5
    v006 = 6; v007 = 7; v008 = 8; v009 = 9; v010 = 10
    # ... (up to v100)
    v096 = 96; v097 = 97; v098 = 98; v099 = 99; v100 = 100

    total = 0
    i = 0
    while i < n {
        total = total + v050  # lookup in middle
        i = i + 1
    }
    return total
}
# Run: bench_many_vars(1_000_000)
```

### 2.3 Function Call Benchmarks

```graphoid
# bench_call_simple.gr
# Measure: simple function call overhead
fn add(a, b) { return a + b }

fn bench_call_simple(n) {
    total = 0
    i = 0
    while i < n {
        total = add(total, 1)
        i = i + 1
    }
    return total
}
# Run: bench_call_simple(1_000_000)

# bench_call_recursive.gr
# Measure: recursive call overhead
fn fib(n) {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
# Run: fib(30)  # Fixed input, measure total time

# bench_call_closure.gr
# Measure: closure call overhead
fn make_adder(x) {
    return fn(y) { return x + y }
}

fn bench_call_closure(n) {
    add5 = make_adder(5)
    total = 0
    i = 0
    while i < n {
        total = add5(total)
        i = i + 1
    }
    return total
}
# Run: bench_call_closure(1_000_000)

# bench_call_deep.gr
# Measure: deep call stack
fn deep(n, acc) {
    if n <= 0 { return acc }
    return deep(n - 1, acc + 1)
}
# Run: deep(10000, 0)  # Measure time for deep recursion
```

### 2.4 Control Flow Benchmarks

```graphoid
# bench_if_simple.gr
fn bench_if(n) {
    total = 0
    i = 0
    while i < n {
        if i % 2 == 0 {
            total = total + 1
        } else {
            total = total + 2
        }
        i = i + 1
    }
    return total
}
# Run: bench_if(1_000_000)

# bench_match.gr
fn classify(x) {
    match x % 4 {
        0 => 1
        1 => 2
        2 => 3
        _ => 4
    }
}

fn bench_match(n) {
    total = 0
    i = 0
    while i < n {
        total = total + classify(i)
        i = i + 1
    }
    return total
}
# Run: bench_match(1_000_000)

# bench_while.gr
fn bench_while(n) {
    total = 0
    i = 0
    while i < n {
        total = total + i
        i = i + 1
    }
    return total
}
# Run: bench_while(10_000_000)

# bench_for.gr
fn bench_for(n) {
    total = 0
    for i in range(0, n) {
        total = total + i
    }
    return total
}
# Run: bench_for(1_000_000)
```

### 2.5 Collection Benchmarks

```graphoid
# bench_list_append.gr
fn bench_list_append(n) {
    lst = []
    i = 0
    while i < n {
        lst.append(i)
        i = i + 1
    }
    return lst.length()
}
# Run: bench_list_append(100_000)

# bench_list_index.gr
fn bench_list_index(n) {
    lst = range(0, 1000).to_list()
    total = 0
    i = 0
    while i < n {
        total = total + lst[i % 1000]
        i = i + 1
    }
    return total
}
# Run: bench_list_index(1_000_000)

# bench_list_map.gr
fn bench_list_map(n) {
    lst = range(0, n).to_list()
    return lst.map(x => x * 2).sum()
}
# Run: bench_list_map(100_000)

# bench_map_access.gr
fn bench_map_access(n) {
    m = {}
    for i in range(0, 100) {
        m[i.to_string()] = i
    }
    total = 0
    i = 0
    while i < n {
        key = (i % 100).to_string()
        total = total + m[key]
        i = i + 1
    }
    return total
}
# Run: bench_map_access(1_000_000)
```

### 2.6 Graph Operation Benchmarks

```graphoid
# bench_graph_create.gr
fn bench_graph_create(n) {
    g = graph{}
    i = 0
    while i < n {
        g.add_node(i.to_string(), i)
        i = i + 1
    }
    return g.nodes().length()
}
# Run: bench_graph_create(10_000)

# bench_graph_edge.gr
fn bench_graph_edge(n) {
    g = graph{}
    # Create nodes first
    for i in range(0, 100) {
        g.add_node(i.to_string(), i)
    }
    # Add edges
    i = 0
    while i < n {
        from = (i % 100).to_string()
        to = ((i + 1) % 100).to_string()
        g.add_edge(from, to, "e" + i.to_string())
        i = i + 1
    }
    return g.edges().length()
}
# Run: bench_graph_edge(10_000)

# bench_graph_traverse.gr
fn bench_graph_traverse(n) {
    # Create a chain graph
    g = graph{}
    for i in range(0, 100) {
        g.add_node(i.to_string(), i)
        if i > 0 {
            g.add_edge((i-1).to_string(), i.to_string(), "next")
        }
    }

    # Traverse n times
    total = 0
    i = 0
    while i < n {
        current = "0"
        while current != "99" {
            neighbors = g.neighbors(current)
            if neighbors.length() > 0 {
                current = neighbors[0]
            } else {
                break
            }
        }
        total = total + 1
        i = i + 1
    }
    return total
}
# Run: bench_graph_traverse(1_000)

# bench_graph_pattern.gr
fn bench_graph_pattern(n) {
    g = graph{}
    # Create a graph with pattern
    for i in range(0, 50) {
        g.add_node("a" + i.to_string(), { type: "A", value: i })
        g.add_node("b" + i.to_string(), { type: "B", value: i * 2 })
        g.add_edge("a" + i.to_string(), "b" + i.to_string(), "connects")
    }

    # Pattern match n times
    total = 0
    i = 0
    while i < n {
        matches = g.match({ from: { type: "A" }, edge: "connects", to: { type: "B" } })
        total = total + matches.length()
        i = i + 1
    }
    return total
}
# Run: bench_graph_pattern(1_000)
```

---

## Part 3: Memory Benchmarks

### 3.1 Memory Measurement Infrastructure

```rust
// benchmarks/memory.rs

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            let current = ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
            PEAK.fetch_max(current, Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

pub fn memory_stats() -> (usize, usize) {
    (ALLOCATED.load(Ordering::SeqCst), PEAK.load(Ordering::SeqCst))
}

pub fn reset_peak() {
    PEAK.store(ALLOCATED.load(Ordering::SeqCst), Ordering::SeqCst);
}
```

### 3.2 Memory Benchmarks

```graphoid
# mem_bench_scope.gr
# Measure: memory per scope level
fn create_scopes(depth) {
    if depth <= 0 { return 0 }
    x = depth  # One variable per scope
    return create_scopes(depth - 1)
}
# Run: create_scopes(100), measure memory
# Calculate: bytes per scope

# mem_bench_variables.gr
# Measure: memory per variable
fn create_vars(n) {
    # This would need to be generated
    v1 = 1; v2 = 2; v3 = 3; ... v_n = n
}
# Measure: memory for n variables
# Calculate: bytes per variable

# mem_bench_closure.gr
# Measure: memory per closure
fn create_closures(n) {
    closures = []
    x = 42
    i = 0
    while i < n {
        closures.append(fn() { return x })
        i = i + 1
    }
    return closures
}
# Run: create_closures(1000), measure memory
# Calculate: bytes per closure

# mem_bench_graph_node.gr
# Measure: memory per graph node
fn create_nodes(n) {
    g = graph{}
    i = 0
    while i < n {
        g.add_node(i.to_string(), i)
        i = i + 1
    }
    return g
}
# Run: create_nodes(10000), measure memory
# Calculate: bytes per node

# mem_bench_graph_edge.gr
# Measure: memory per graph edge
fn create_edges(n) {
    g = graph{}
    g.add_node("hub", 0)
    i = 0
    while i < n {
        g.add_node(i.to_string(), i)
        g.add_edge("hub", i.to_string(), "connects")
        i = i + 1
    }
    return g
}
# Run: create_edges(10000), measure memory
# Calculate: bytes per edge
```

---

## Part 4: Stress Tests

### 4.1 Scale Tests

```graphoid
# stress_many_vars.gr
# Test: 10,000 variables in one scope
fn stress_many_vars() {
    # Generate 10,000 variable assignments
    # v0001 = 1; v0002 = 2; ... v10000 = 10000
    # Then access some of them
    return v5000 + v9999
}

# stress_deep_scope.gr
# Test: 1,000 nested scopes
fn stress_deep_scope(n) {
    if n <= 0 { return 0 }
    x = n
    return stress_deep_scope(n - 1) + x
}
# Run: stress_deep_scope(1000)

# stress_deep_recursion.gr
# Test: deep recursion without stack overflow
fn stress_recursion(n, acc) {
    if n <= 0 { return acc }
    return stress_recursion(n - 1, acc + 1)
}
# Run: stress_recursion(100000, 0)

# stress_large_graph.gr
# Test: graph with 100,000 nodes
fn stress_large_graph() {
    g = graph{}
    for i in range(0, 100000) {
        g.add_node(i.to_string(), i)
    }
    # Add random edges
    for i in range(0, 100000) {
        j = (i * 7) % 100000  # pseudo-random
        g.add_edge(i.to_string(), j.to_string(), "link")
    }
    return g.nodes().length()
}

# stress_long_running.gr
# Test: sustained execution (memory leaks, GC pressure)
fn stress_long_running(minutes) {
    end_time = time.now() + minutes * 60
    iterations = 0
    while time.now() < end_time {
        # Create and discard objects
        lst = range(0, 1000).to_list()
        m = {}
        for i in range(0, 100) {
            m[i.to_string()] = lst[i]
        }
        g = graph{}
        for i in range(0, 50) {
            g.add_node(i.to_string(), m[(i % 100).to_string()])
        }
        iterations = iterations + 1
    }
    return iterations
}
# Run: stress_long_running(5)  # 5 minutes
# Monitor: memory usage over time, should not grow
```

### 4.2 Concurrency Stress (Future)

```graphoid
# stress_concurrent_access.gr (Phase 15+)
fn stress_concurrent() {
    shared_graph = graph{}

    # Spawn multiple workers accessing same graph
    workers = []
    for i in range(0, 10) {
        workers.append(spawn(fn() {
            for j in range(0, 1000) {
                shared_graph.add_node(i.to_string() + "_" + j.to_string(), j)
            }
        }))
    }

    # Wait for all
    for w in workers { w.join() }

    return shared_graph.nodes().length()  # Should be 10,000
}
```

---

## Part 5: Performance Targets

### 5.1 Baseline Measurements

Run all benchmarks against **current implementation** to establish baseline.

```bash
GRAPHOID_RUNTIME=current cargo run --release -- benchmarks/
```

Record:
- Mean time per operation
- Standard deviation
- Operations per second
- Memory usage

### 5.2 Target Thresholds

| Category | Metric | Target | Acceptable | Failure |
|----------|--------|--------|------------|---------|
| **Variable Lookup** | local | ≤1.2x baseline | ≤1.5x | >2x |
| | nested (1 level) | ≤1.3x baseline | ≤1.5x | >2x |
| | deep (5 levels) | ≤1.5x baseline | ≤2x | >3x |
| | many vars (100) | ≤1.3x baseline | ≤1.5x | >2x |
| **Function Calls** | simple | ≤1.2x baseline | ≤1.5x | >2x |
| | recursive | ≤1.3x baseline | ≤1.5x | >2x |
| | closure | ≤1.5x baseline | ≤2x | >3x |
| **Control Flow** | if/else | ≤1.1x baseline | ≤1.3x | >1.5x |
| | while loop | ≤1.1x baseline | ≤1.3x | >1.5x |
| | pattern match | ≤1.3x baseline | ≤1.5x | >2x |
| **Collections** | list append | ≤1.2x baseline | ≤1.5x | >2x |
| | list index | ≤1.1x baseline | ≤1.3x | >1.5x |
| | map access | ≤1.2x baseline | ≤1.5x | >2x |
| **Graph Ops** | create node | ≤1.0x baseline | ≤1.2x | >1.5x |
| | add edge | ≤1.0x baseline | ≤1.2x | >1.5x |
| | traverse | ≤1.0x baseline | ≤1.2x | >1.5x |
| **Memory** | per scope | ≤1.5x baseline | ≤2x | >3x |
| | per variable | ≤1.5x baseline | ≤2x | >3x |
| | per closure | ≤1.5x baseline | ≤2x | >3x |

### 5.3 Absolute Targets (vs Python)

For comparison with Python on equivalent benchmarks:

| Benchmark | Graphoid Target | Python Baseline |
|-----------|-----------------|-----------------|
| fib(30) | ≤2x Python | measure |
| loop 1M | ≤1.5x Python | measure |
| list append 100K | ≤1.5x Python | measure |
| dict access 1M | ≤1.5x Python | measure |

---

## Part 6: Benchmark Runner

### 6.1 Command Line Interface

```bash
# Run all benchmarks
graphoid bench

# Run specific category
graphoid bench --category lookup
graphoid bench --category calls
graphoid bench --category memory

# Compare two implementations
graphoid bench --compare current,graph

# Output formats
graphoid bench --format table
graphoid bench --format json
graphoid bench --format csv

# Detailed output
graphoid bench --verbose
```

### 6.2 Output Format

```
Graphoid Benchmark Suite v1.0
Runtime: graph (experimental)
Date: 2026-01-15

VARIABLE LOOKUP
---------------
bench_lookup_local      1,234,567 ops/sec  (1.15x baseline) ✓
bench_lookup_nested     987,654 ops/sec    (1.28x baseline) ✓
bench_lookup_deep       456,789 ops/sec    (1.67x baseline) ⚠
bench_lookup_many_vars  1,111,111 ops/sec  (1.21x baseline) ✓

FUNCTION CALLS
--------------
bench_call_simple       876,543 ops/sec    (1.18x baseline) ✓
bench_call_recursive    fib(30) in 45ms    (1.35x baseline) ✓
bench_call_closure      654,321 ops/sec    (1.52x baseline) ⚠

MEMORY
------
per_scope              128 bytes           (1.45x baseline) ✓
per_variable           48 bytes            (1.33x baseline) ✓
per_closure            96 bytes            (1.78x baseline) ⚠

SUMMARY
-------
Passed: 18/24 (75%)
Warnings: 4/24 (17%)
Failed: 2/24 (8%)

Overall: ACCEPTABLE (with noted concerns)
```

---

## Part 7: Graph-Specific Capability Tests

These tests validate that graph-centricity provides expected benefits.

### 7.1 Introspection Tests

```graphoid
# test_introspect_namespace.gr
x = 5
y = 10
fn add(a, b) { return a + b }

# Should be able to inspect namespace as graph
ns = debug.namespace()
assert(ns.has_node("x"), "namespace has x")
assert(ns.has_node("y"), "namespace has y")
assert(ns.has_node("add"), "namespace has add")
assert(ns.get_binding("x") == 5, "binding value")

# test_introspect_scope_chain.gr
fn outer() {
    x = 1
    fn inner() {
        y = 2
        chain = debug.scope_chain()
        assert(chain.length() == 3, "global -> outer -> inner")
        assert(chain[0].name == "inner", "current scope")
        assert(chain[1].name == "outer", "parent scope")
        assert(chain[2].name == "global", "root scope")
    }
    inner()
}
outer()
```

### 7.2 Hot Reload Tests (Future)

```graphoid
# test_hot_reload.gr
fn greet(name) {
    return "hello " + name
}

assert(greet("world") == "hello world", "initial")

# Hot reload the function
debug.reload_function("greet", fn(name) {
    return "hi " + name
})

assert(greet("world") == "hi world", "after reload")
```

### 7.3 Function-as-Graph Tests

```graphoid
# test_function_graph.gr
fn add(a, b) { return a + b }

# Function should be inspectable as a graph
fn_graph = debug.function_graph(add)
assert(fn_graph.type == :subgraph, "function is a subgraph")
assert(fn_graph.has_node("param:a"), "has param a")
assert(fn_graph.has_node("param:b"), "has param b")
assert(fn_graph.has_node("body"), "has body subgraph")

# Body should contain the return and binary op
body = fn_graph.get_subgraph("body")
assert(body.has_node({ type: :return }), "has return")
assert(body.has_node({ type: :binary, op: "+" }), "has binary +")

# test_closure_graph.gr
fn make_adder(x) {
    return fn(y) { return x + y }
}
add5 = make_adder(5)

# Closure should have edge to captured variable
closure_graph = debug.function_graph(add5)
captures = closure_graph.edges({ label: "captures" })
assert(captures.length() == 1, "one captured variable")
assert(captures[0].target.name == "x", "captured x")

# test_composition_graph.gr
fn double(x) { return x * 2 }
fn inc(x) { return x + 1 }

# Compose creates new subgraph connecting the two
composed = compose(double, inc)  # composed(x) = double(inc(x))
comp_graph = debug.function_graph(composed)

# Should see inc's body feeding into double's body
assert(comp_graph.has_edge({ from: "inc:result", to: "double:input" }), "composed structure")

# test_inline_graph.gr
fn small(x) { return x + 1 }
fn caller(y) { return small(y) * 2 }

# Before inlining
caller_graph = debug.function_graph(caller)
assert(caller_graph.has_node({ type: :call, callee: "small" }), "has call node")

# After inlining
inlined = optimizer.inline(caller, small)
inlined_graph = debug.function_graph(inlined)
assert(not inlined_graph.has_node({ type: :call, callee: "small" }), "call removed")
assert(inlined_graph.has_node({ type: :binary, op: "+" }), "inlined body present")
```

### 7.4 Graph Optimization Tests

```graphoid
# test_constant_folding.gr
# The optimizer should fold 3 + 4 to 7
fn test_fold() {
    return 3 + 4
}

ast = debug.get_ast(test_fold)
# After optimization, should have Const(7), not Binary(+, Const(3), Const(4))
assert(ast.body[0].type == :const, "folded to constant")
assert(ast.body[0].value == 7, "correct value")
```

---

## Part 8: Execution Checklist

### Before Each Experiment

- [ ] Run all correctness tests against current implementation
- [ ] Record baseline benchmarks
- [ ] Document system specs (CPU, RAM, OS)
- [ ] Clear caches, ensure consistent environment

### During Experiment

- [ ] Implement experimental runtime
- [ ] Run correctness tests (must pass 100%)
- [ ] Run performance benchmarks
- [ ] Run memory benchmarks
- [ ] Run stress tests
- [ ] Document any failures or anomalies

### After Experiment

- [ ] Compare results to baseline
- [ ] Identify bottlenecks
- [ ] Document lessons learned
- [ ] Decide: iterate, proceed, or abandon

---

## Appendix: Benchmark File Locations

```
benchmarks/
├── correctness/
│   ├── binding/
│   ├── execution/
│   ├── collections/
│   └── modules/
├── performance/
│   ├── lookup/
│   ├── calls/
│   ├── control/
│   ├── collections/
│   └── graphs/
├── memory/
│   ├── scope/
│   ├── variables/
│   ├── closures/
│   └── graphs/
├── stress/
│   ├── scale/
│   └── duration/
├── capabilities/
│   ├── introspection/
│   ├── hot_reload/
│   └── optimization/
└── infrastructure/
    ├── runner.rs
    ├── memory.rs
    └── compare.rs
```
