# Phase 32: Self-Hosting

**Duration**: 30-45 days
**Priority**: Ultimate Goal
**Dependencies**: Phase 29, Phase 30 or 31 (need compilation capability)
**Status**: Future

---

## Goal

Implement the Graphoid compiler entirely in Graphoid, achieving complete self-sufficiency:
- **Zero Rust dependency** - Delete the Rust implementation
- **Zero external dependencies** - No libc, no LLVM
- **Bootstrap capability** - Graphoid compiles itself

---

## What is Self-Hosting?

A self-hosted compiler can compile its own source code:

```
+---------------------------------------------------------------------+
|                      Self-Hosting Bootstrap                          |
|                                                                     |
|  Stage 0:  Rust compiler compiles Graphoid compiler v1              |
|            (Graphoid source -> Rust -> Native binary)               |
|                                                                     |
|  Stage 1:  Graphoid compiler v1 compiles Graphoid compiler v2       |
|            (Graphoid source -> Graphoid v1 -> Native binary)        |
|                                                                     |
|  Stage 2:  Graphoid compiler v2 compiles Graphoid compiler v3       |
|            (Should produce identical binary to Stage 1)             |
|                                                                     |
|  Success:  If Stage 1 binary == Stage 2 binary, self-hosting works! |
+---------------------------------------------------------------------+
```

---

## Why Self-Hosting Matters

| Benefit | Description |
|---------|-------------|
| **Independence** | No dependency on Rust, LLVM, or any external toolchain |
| **Dogfooding** | Forces Graphoid to be powerful enough for real systems programming |
| **Simplicity** | One language for everything - no polyglot toolchain |
| **Portability** | Graphoid binaries can run anywhere, compile anywhere |
| **Credibility** | Self-hosted languages are taken seriously |

### Languages That Self-Host

| Language | Self-Hosted Since | Bootstrap From |
|----------|-------------------|----------------|
| C | 1973 | Assembly |
| Go | 2015 (Go 1.5) | C (original) |
| Rust | 2011 | OCaml (original) |
| Zig | In progress | C++ (LLVM) |
| Nim | Yes | C |

---

## Graph-Centric Self-Hosting

### Why Self-Hosting is Graph-Centric

Self-hosting Graphoid in Graphoid demonstrates the language's power at every level:

```
┌─────────────────────────────────────────────────────────────────┐
│               Self-Hosted Compiler Architecture                  │
│                                                                 │
│  Source Code (.gr)                                               │
│       ↓                                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Lexer (Graphoid)     - Token stream as linked list graph   ││
│  └─────────────────────────────────────────────────────────────┘│
│       ↓                                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Parser (Graphoid)    - AST as tree graph with parent edges ││
│  └─────────────────────────────────────────────────────────────┘│
│       ↓                                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Analyzer (Graphoid)  - Symbol table as namespace graph     ││
│  └─────────────────────────────────────────────────────────────┘│
│       ↓                                                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Graph IR (Graphoid)  - Program representation as graph     ││
│  └─────────────────────────────────────────────────────────────┘│
│       ↓              ↓                                          │
│  ┌──────────┐   ┌──────────┐                                    │
│  │ Scalar   │   │  Graph   │                                    │
│  │ Bytecode │   │ Runtime  │                                    │
│  └──────────┘   └──────────┘                                    │
│       ↓              ↓                                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  Native Code (Graphoid) - Direct syscalls, no libc          ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Graph IR in Self-Hosted Compiler

The self-hosted compiler must support the dual-path model from Phase 29:

```graphoid
# stdlib/compiler/graph_ir.gr

class GraphIR {
    nodes = graph{}       # IR operations as nodes
    edges = []            # Data flow and control flow edges
    scalar_ops = []       # Operations compiled to bytecode
    graph_ops = []        # Operations using graph runtime

    fn analyze(ast) {
        # Build IR from AST
        for stmt in ast.statements {
            node = ir_node_for(stmt)
            nodes.add_node(node.id, node)

            # Classify: scalar or graph operation?
            if is_graph_operation(stmt) {
                graph_ops.append(node.id)
            } else {
                scalar_ops.append(node.id)
            }
        }
        return self
    }

    fn is_graph_operation(stmt) {
        # Check if statement involves graphs
        match stmt {
            Ast.MethodCall { receiver, method } => {
                graph_methods = ["add_node", "add_edge", "traverse",
                                 "query", "neighbors", "paths"]
                return graph_methods.contains(method)
            }
            Ast.GraphLiteral { } => return true
            Ast.Query { } => return true
            _ => return false
        }
    }

    fn optimize() {
        # Graph-based optimization passes
        inline_small_functions()
        eliminate_dead_code()
        fold_constants()
        specialize_graph_queries()  # Important for graph ops!
    }
}
```

### Five-Layer Architecture in Self-Hosted Compiler

The compiler must preserve all five layers when generating code:

```graphoid
# stdlib/compiler/codegen/layers.gr

class LayerPreservingCodegen {

    fn compile_graph_creation(graph_literal) {
        # Preserve all five layers in generated code

        # Layer 1: Data Layer - the actual nodes/edges
        emit_call("__gr_graph_create")
        for node in graph_literal.nodes {
            emit_call("__gr_graph_add_node", node.id, node.value)
        }
        for edge in graph_literal.edges {
            emit_call("__gr_graph_add_edge",
                      edge.from, edge.to, edge.label)
        }

        # Layer 2: Behavior Layer - transformations
        for behavior in graph_literal.behaviors {
            emit_call("__gr_behavior_attach", behavior.name, behavior.config)
        }

        # Layer 3: Control Layer - rules and constraints
        for rule in graph_literal.rules {
            emit_call("__gr_rule_add", rule.name)
        }

        # Layer 4: Metadata Layer
        if graph_literal.metadata.length() > 0 {
            for [key, value] in graph_literal.metadata {
                emit_call("__gr_metadata_set", key, value)
            }
        }

        # Layer 5: Effect Layer - observers
        for observer in graph_literal.observers {
            emit_call("__gr_observer_register", observer)
        }
    }

    fn compile_graph_query(query) {
        # Query compilation preserves layer semantics

        # Check rules before query execution
        emit_call("__gr_query_validate", query.pattern)

        # Execute query with behavior application
        emit_call("__gr_query_execute", query.pattern)

        # Apply any transformations
        if query.transform {
            emit_call("__gr_query_transform", query.transform)
        }
    }
}
```

---

## Components to Rewrite

### Current Rust Implementation

```
src/
+-- lexer/          ->  stdlib/compiler/lexer.gr
+-- parser/         ->  stdlib/compiler/parser.gr
+-- ast/            ->  stdlib/compiler/ast.gr
+-- execution/      ->  (replaced by native code)
+-- values/         ->  stdlib/compiler/values.gr
+-- graph/          ->  stdlib/graph.gr (already partly done)
+-- stdlib/         ->  stdlib/ (already Graphoid)
```

### New Graphoid Implementation

```
stdlib/compiler/
+-- lexer.gr        # Tokenization
+-- parser.gr       # AST construction
+-- ast.gr          # AST node types
+-- analyzer.gr     # Semantic analysis
+-- bytecode.gr     # Bytecode generation
+-- codegen/
|   +-- wasm.gr     # WASM backend
|   +-- native.gr   # Native backend
+-- linker.gr       # Executable generation
+-- driver.gr       # CLI interface

stdlib/runtime/
+-- memory.gr       # Allocator, GC
+-- syscall.gr      # System call wrappers
+-- io.gr           # I/O primitives
+-- panic.gr        # Error handling
```

---

## Implementation Strategy

### Phase 32a: Lexer in Graphoid

```graphoid
# stdlib/compiler/lexer.gr

class Token {
    type = none      # :identifier, :number, :string, :keyword, etc.
    value = none     # The actual value
    line = 0
    column = 0
}

class Lexer {
    source = ""
    pos = 0
    line = 1
    column = 1

    fn new(source) {
        self.source = source
        return self
    }

    fn next_token() {
        skip_whitespace()

        if at_end() {
            return Token { type: :eof, line: line, column: column }
        }

        c = peek()

        if c.is_digit() {
            return scan_number()
        }

        if c.is_alpha() or c == "_" {
            return scan_identifier()
        }

        if c == "\"" {
            return scan_string()
        }

        # Operators and punctuation
        match c {
            "+" => return single_char_token(:plus)
            "-" => return single_char_token(:minus)
            "*" => return single_char_token(:star)
            "/" => return single_char_token(:slash)
            # ... etc
        }
    }

    fn scan_number() {
        start = pos
        while peek().is_digit() {
            advance()
        }
        if peek() == "." and peek_next().is_digit() {
            advance()  # consume '.'
            while peek().is_digit() {
                advance()
            }
        }
        return Token {
            type: :number,
            value: source.slice(start, pos).to_num(),
            line: line,
            column: column
        }
    }

    # ... more methods
}
```

### Phase 32b: Parser in Graphoid

```graphoid
# stdlib/compiler/parser.gr

class Parser {
    tokens = []
    pos = 0

    fn parse() {
        statements = []
        while not at_end() {
            statements.append(parse_statement())
        }
        return Ast.Program { statements: statements }
    }

    fn parse_statement() {
        if check(:fn) {
            return parse_function()
        }
        if check(:if) {
            return parse_if()
        }
        if check(:while) {
            return parse_while()
        }
        if check(:for) {
            return parse_for()
        }
        if check(:return) {
            return parse_return()
        }
        return parse_expression_statement()
    }

    fn parse_expression() {
        return parse_assignment()
    }

    fn parse_assignment() {
        expr = parse_or()
        if match(:equal) {
            value = parse_assignment()
            if expr.type == :identifier {
                return Ast.Assignment { name: expr.name, value: value }
            }
            raise ParseError { message: "Invalid assignment target" }
        }
        return expr
    }

    fn parse_or() {
        expr = parse_and()
        while match(:or) {
            right = parse_and()
            expr = Ast.Binary { op: :or, left: expr, right: right }
        }
        return expr
    }

    # ... precedence climbing continues
}
```

### Phase 32c: Code Generation in Graphoid

```graphoid
# stdlib/compiler/codegen/native.gr

class NativeCodegen {
    code = []           # Machine code bytes
    labels = {}         # Label -> offset
    relocations = []    # Pending relocations

    fn compile(ast) {
        for stmt in ast.statements {
            compile_statement(stmt)
        }
        resolve_relocations()
        return code
    }

    fn compile_statement(stmt) {
        match stmt {
            Ast.Function { name, params, body } => {
                labels[name] = code.length()
                emit_prologue(params.length())
                compile_block(body)
                emit_epilogue()
            }
            Ast.Return { value } => {
                compile_expression(value)
                emit_return()
            }
            # ... etc
        }
    }

    fn compile_expression(expr) {
        match expr {
            Ast.Number { value } => {
                emit_mov_imm(RAX, value)
            }
            Ast.Binary { op: :plus, left, right } => {
                compile_expression(left)
                emit_push(RAX)
                compile_expression(right)
                emit_pop(RBX)
                emit_add(RAX, RBX)
            }
            Ast.Call { name, args } => {
                for arg in args.reverse() {
                    compile_expression(arg)
                    emit_push(RAX)
                }
                emit_call(name)
                emit_add_imm(RSP, args.length() * 8)
            }
            # ... etc
        }
    }

    # x86_64 instruction emitters
    fn emit_mov_imm(reg, value) {
        code.append(0x48)  # REX.W
        code.append(0xB8 + reg)
        code.append_i64(value)
    }

    fn emit_push(reg) {
        code.append(0x50 + reg)
    }

    fn emit_pop(reg) {
        code.append(0x58 + reg)
    }

    fn emit_add(dst, src) {
        code.append(0x48)  # REX.W
        code.append(0x01)
        code.append(0xC0 + dst + src * 8)
    }

    fn emit_call(name) {
        code.append(0xE8)
        relocations.append({
            offset: code.length(),
            target: name,
            type: :rel32
        })
        code.append_i32(0)  # Placeholder
    }

    fn emit_ret() {
        code.append(0xC3)
    }
}
```

### Phase 32d: Runtime in Graphoid

```graphoid
# stdlib/runtime/memory.gr

# Simple bump allocator (no GC initially)
heap_base = 0
heap_ptr = 0
heap_end = 0

fn init_heap(size) {
    heap_base = syscall.mmap(
        0,                          # addr (let kernel choose)
        size,                       # length
        syscall.PROT_READ | syscall.PROT_WRITE,
        syscall.MAP_PRIVATE | syscall.MAP_ANONYMOUS,
        -1,                         # fd
        0                           # offset
    )
    heap_ptr = heap_base
    heap_end = heap_base + size
}

fn alloc(size) {
    aligned = (size + 7) & ~7  # 8-byte alignment
    if heap_ptr + aligned > heap_end {
        grow_heap()
    }
    ptr = heap_ptr
    heap_ptr = heap_ptr + aligned
    return ptr
}

fn grow_heap() {
    # Double the heap
    new_size = (heap_end - heap_base) * 2
    new_base = syscall.mremap(heap_base, heap_end - heap_base, new_size, 1)
    heap_base = new_base
    heap_end = new_base + new_size
}
```

```graphoid
# stdlib/runtime/syscall.gr

# Linux x86_64 syscall numbers
SYS_READ = 0
SYS_WRITE = 1
SYS_OPEN = 2
SYS_CLOSE = 3
SYS_MMAP = 9
SYS_MUNMAP = 11
SYS_EXIT = 60

# Inline assembly for syscalls (special compiler support needed)
fn syscall0(num) {
    return __asm__("syscall", rax: num)
}

fn syscall1(num, arg1) {
    return __asm__("syscall", rax: num, rdi: arg1)
}

fn syscall2(num, arg1, arg2) {
    return __asm__("syscall", rax: num, rdi: arg1, rsi: arg2)
}

fn syscall3(num, arg1, arg2, arg3) {
    return __asm__("syscall", rax: num, rdi: arg1, rsi: arg2, rdx: arg3)
}

fn write(fd, buf, count) {
    return syscall3(SYS_WRITE, fd, buf, count)
}

fn read(fd, buf, count) {
    return syscall3(SYS_READ, fd, buf, count)
}

fn exit(code) {
    syscall1(SYS_EXIT, code)
}
```

---

## Bootstrap Process

### Step 1: Prepare

```bash
# Ensure Rust compiler produces working binaries
cargo build --release
./target/release/graphoid --version
```

### Step 2: Compile Graphoid Compiler with Rust Version

```bash
# Compile the Graphoid compiler (written in Graphoid) using Rust implementation
./target/release/graphoid compile \
    --target native \
    stdlib/compiler/driver.gr \
    -o graphoid-stage1
```

### Step 3: First Bootstrap

```bash
# Compile the Graphoid compiler using Stage 1
./graphoid-stage1 compile \
    --target native \
    stdlib/compiler/driver.gr \
    -o graphoid-stage2
```

### Step 4: Verify Bootstrap

```bash
# Compile again with Stage 2
./graphoid-stage2 compile \
    --target native \
    stdlib/compiler/driver.gr \
    -o graphoid-stage3

# Compare binaries
diff graphoid-stage2 graphoid-stage3
# Should be identical!
```

### Step 5: Delete Rust

```bash
# Once bootstrap is verified
rm -rf src/           # Remove Rust source
rm Cargo.toml         # Remove Rust build config
mv graphoid-stage2 graphoid  # The new compiler

# Graphoid is now self-hosting!
```

---

## Implementation Plan

### Month 1: Lexer & Parser

| Week | Task |
|------|------|
| 1 | Lexer in Graphoid (tokenization) |
| 2 | Parser in Graphoid (AST construction) |
| 3 | AST types, semantic analysis basics |
| 4 | Testing, edge cases, error messages |

### Month 2: Bytecode Compiler

| Week | Task |
|------|------|
| 5 | Bytecode generation (expressions) |
| 6 | Bytecode generation (statements, functions) |
| 7 | Bytecode generation (collections, graphs) |
| 8 | VM execution (verify bytecode works) |

### Month 3: Native Code Generation

| Week | Task |
|------|------|
| 9 | x86_64 instruction encoding |
| 10 | Code generation (expressions, control flow) |
| 11 | Code generation (functions, closures) |
| 12 | ELF generation, linking |

### Month 4: Runtime & Bootstrap

| Week | Task |
|------|------|
| 13 | Runtime library (memory, I/O) |
| 14 | Syscall interface |
| 15 | Bootstrap testing |
| 16 | Polish, documentation, celebration |

---

## Success Criteria

### Core Components
- [ ] Lexer written in Graphoid
- [ ] Parser written in Graphoid
- [ ] Bytecode compiler written in Graphoid
- [ ] Native code generator written in Graphoid
- [ ] Runtime library written in Graphoid
- [ ] Syscall interface (Linux x86_64)
- [ ] ELF executable generation

### Graph-Centric Requirements
- [ ] Graph IR analysis (classify scalar vs graph operations)
- [ ] Dual-path compilation (scalar → bytecode, graph → runtime)
- [ ] Five-Layer Architecture preserved in generated code
- [ ] Graph query optimization in self-hosted compiler
- [ ] Graph runtime functions implemented in pure Graphoid
- [ ] Compiler internal data structures use Graphoid graphs

### Bootstrap Verification
- [ ] Successful three-stage bootstrap
- [ ] Stage 2 and Stage 3 binaries identical
- [ ] All tests pass under self-hosted compiler
- [ ] Graph operations work identically to Rust implementation

### Completion
- [ ] Rust code deleted from repository
- [ ] Documentation complete

---

## Challenges

### 1. Inline Assembly

For syscalls, we need some way to emit raw assembly:

```graphoid
# Option A: Magic function
result = __asm__("syscall", rax: 1, rdi: 1, rsi: buf, rdx: len)

# Option B: Special syntax
asm {
    mov rax, 1
    mov rdi, 1
    mov rsi, buf
    mov rdx, len
    syscall
}
```

### 2. Pointer Arithmetic

Low-level code needs pointer math:

```graphoid
# Need way to work with raw pointers
ptr = alloc(100)
ptr[0] = 65           # Write byte
ptr = ptr + 8         # Pointer arithmetic
value = deref(ptr)    # Read
```

### 3. Bootstrap Chicken-and-Egg

Need to be careful about features:
- Can't use features the compiler can't yet compile
- Start with minimal subset, gradually add features

### 4. Testing Without Rust

Once Rust is deleted, need Graphoid-based test runner:

```graphoid
# stdlib/test/runner.gr
fn run_tests(test_files) {
    passed = 0
    failed = 0
    for file in test_files {
        result = run_test(file)
        if result.success {
            passed = passed + 1
        } else {
            failed = failed + 1
            print("FAIL: " + file + " - " + result.message)
        }
    }
    print("Passed: " + passed.to_string() + "/" + (passed + failed).to_string())
}
```

---

## Post-Self-Hosting

Once self-hosted, development continues in pure Graphoid:

| Enhancement | Description |
|-------------|-------------|
| Optimizations | Better code generation, inlining |
| More targets | aarch64, WASM from Graphoid |
| Better GC | Tracing collector in Graphoid |
| Better errors | Rich diagnostics |
| Tooling | Debugger, profiler in Graphoid |

---

## Related Documents

- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Compilation strategy
- [PHASE_30_WASM_COMPILATION.md](PHASE_30_WASM_COMPILATION.md) - Alternative target
- [PHASE_31_NATIVE_COMPILATION.md](PHASE_31_NATIVE_COMPILATION.md) - Native compilation foundation

---

## Inspiration

- **Go's Bootstrap** - Rewrote compiler from C to Go in 2015
- **Rust's Bootstrap** - Originally OCaml, now self-hosted
- **Zig's Bootstrap** - Working toward self-hosting from C++
- **Nim's Bootstrap** - Self-hosted via C intermediate

---

## The End State

```
+---------------------------------------------------------------------+
|                                                                     |
|   $ graphoid --version                                              |
|   Graphoid 2.0.0 (self-hosted)                                      |
|                                                                     |
|   $ file $(which graphoid)                                          |
|   graphoid: ELF 64-bit LSB executable, x86-64                       |
|   (no external dependencies)                                        |
|                                                                     |
|   $ graphoid compile compiler/driver.gr -o graphoid-new             |
|   $ diff graphoid graphoid-new                                      |
|   (no differences - perfect bootstrap)                              |
|                                                                     |
|   Graphoid compiles itself!                                         |
|                                                                     |
+---------------------------------------------------------------------+
```
