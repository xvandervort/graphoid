# REPL Navigation Features

## Overview

Glang's REPL now includes modern command-line navigation features that make interactive development much more comfortable:

- **↑/↓ Arrow Keys** - Navigate through command history
- **←/→ Arrow Keys** - Move cursor within the current line  
- **Tab Completion** - Auto-complete commands, variables, and graph types
- **Persistent History** - Commands saved between sessions
- **Smart Completion** - Context-aware suggestions

## Command History

### Usage
- Press **↑** to go back through previous commands
- Press **↓** to go forward through command history
- Commands are saved to `~/.glang_history` and persist between sessions
- Duplicate consecutive commands are automatically filtered out

### Example Session
```bash
glang> create fruits [apple, banana, cherry]
glang> show fruits
glang> append orange
glang> namespace
# Press ↑ to cycle: namespace → append orange → show fruits → create fruits...
# Press ↓ to cycle forward through history
```

## Tab Completion

### Command Completion
Start typing any command and press **Tab** to complete:

```bash
glang> cr<Tab>     → create
glang> na<Tab>     → namespace  
glang> he<Tab>     → help
glang> tra<Tab>    → traverse
```

### Variable Name Completion
After commands that work with variables, **Tab** completes variable names:

```bash
glang> create fruits [apple, banana]
glang> create numbers [1, 2, 3] 
glang> show fr<Tab>        → show fruits
glang> delete nu<Tab>      → delete numbers
glang> info f<Tab>         → info fruits
```

### Graph Type Completion
When creating graphs, **Tab** completes graph types:

```bash
glang> create test l<Tab>      → create test linear
glang> create graph di<Tab>    → create graph directed
glang> create tree t<Tab>      → create tree tree
```

Available graph types: `linear`, `directed`, `tree`, `cyclic`, `weighted`, `undirected`

## Cursor Movement

### Within Current Line
- **←/→ Arrow Keys** - Move cursor left/right within the current command
- **Home/Ctrl+A** - Move to beginning of line
- **End/Ctrl+E** - Move to end of line
- **Ctrl+W** - Delete word backward
- **Ctrl+K** - Delete from cursor to end of line

### Example
```bash
glang> create fruits [apple, banana, cherry]
       ^cursor can move anywhere in this line
```

## Smart Features

### Context-Aware Completion
Tab completion is context-aware:

1. **First word**: Completes all available commands
2. **After `show/traverse/delete/info`**: Completes with existing variable names
3. **After `create <name>`**: Completes with graph types
4. **Unknown contexts**: Returns empty (no meaningless suggestions)

### History Management
- **Automatic deduplication**: Won't store consecutive identical commands
- **Persistent storage**: History saved to `~/.glang_history`
- **Limited size**: Keeps last 1000 commands
- **Graceful fallback**: Works without readline if not available

## Keyboard Shortcuts

### Standard Readline Shortcuts
- **Ctrl+C** - Interrupt current command / Exit REPL
- **Ctrl+D** - EOF signal (exits REPL)
- **Ctrl+L** - Clear screen (if supported)
- **Ctrl+R** - Reverse history search (if supported)

### Glang-Specific
- **Tab** - Smart completion based on context
- **↑/↓** - Command history navigation
- **help** - Shows navigation help section

## Examples in Practice

### Efficient Workflow
```bash
# Create some data
glang> create fruits [apple, banana, cherry]
glang> create numbers [1, 2, 3, 4, 5]

# Use tab completion for efficiency  
glang> sh<Tab> fr<Tab>     → show fruits
[apple] -> [banana] -> [cherry]

glang> tr<Tab> nu<Tab>     → traverse numbers  
Traversal: [1, 2, 3, 4, 5]

# Use arrow keys to repeat/modify previous commands
glang> ↑                  → traverse numbers (previous command)
glang> ←←←←←←←             → move cursor to "numbers"
glang> Backspace×7        → delete "numbers"  
glang> fr<Tab>            → traverse fruits
Traversal: ['apple', 'banana', 'cherry']
```

### Exploring Commands
```bash
# Type partial command and tab to see options
glang> <Tab><Tab>         → shows all commands
glang> c<Tab>             → create
glang> create test <Tab>  → shows graph types
```

## Technical Implementation

### Readline Integration
- Uses Python's `readline` module when available
- Graceful fallback when readline is not installed
- Cross-platform compatibility (Linux/Mac/Windows with appropriate setup)

### History File
- Location: `~/.glang_history`
- Format: Plain text, one command per line
- Automatic creation and management
- Safe handling of file I/O errors

### Completion Engine
- Parses current line to understand context
- Provides relevant suggestions based on command structure
- Integrates with glang's variable graph for dynamic completion

## Benefits

### Developer Experience
- **Faster interaction** - Tab completion reduces typing
- **Less repetition** - History navigation for common commands
- **Fewer typos** - Auto-completion prevents spelling mistakes
- **Natural feel** - Familiar readline behavior from other tools

### Learning Aid
- **Discovery** - Tab completion helps discover available commands
- **Exploration** - Easy to try variations of commands
- **Reference** - Help system shows navigation features

The navigation improvements make glang's REPL feel like a modern, professional development tool while maintaining the unique graph-based philosophy that makes it special! 🚀