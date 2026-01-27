# fs Module

The `fs` module provides file system operations for reading, writing, and inspecting files and directories.

## Import

```graphoid
import "fs"
```

## Functions

### File Operations

#### `fs.open(path, mode) -> file_handle`

Opens a file and returns a file handle for subsequent operations.

**Parameters:**
- `path` - Path to the file
- `mode` - Open mode: `"r"` (read), `"w"` (write), `"a"` (append)

**Returns:** A numeric file handle

```graphoid
handle = fs.open("data.txt", "r")
```

#### `fs.read(handle, bytes) -> string`

Reads up to `bytes` bytes from an open file.

**Parameters:**
- `handle` - File handle from `fs.open()`
- `bytes` - Maximum number of bytes to read

**Returns:** String containing the file contents

```graphoid
handle = fs.open("data.txt", "r")
content = fs.read(handle, 1024)
fs.close(handle)
```

#### `fs.write(handle, content) -> none`

Writes content to an open file.

**Parameters:**
- `handle` - File handle from `fs.open()`
- `content` - String to write

```graphoid
handle = fs.open("output.txt", "w")
fs.write(handle, "Hello, World!\n")
fs.close(handle)
```

#### `fs.close(handle) -> none`

Closes an open file handle.

**Parameters:**
- `handle` - File handle from `fs.open()`

```graphoid
handle = fs.open("data.txt", "r")
content = fs.read(handle, 1024)
fs.close(handle)
```

### Directory Operations

#### `fs.list_dir(path) -> list`

Lists the contents of a directory.

**Parameters:**
- `path` - Path to the directory

**Returns:** A sorted list of filenames (strings)

```graphoid
files = fs.list_dir(".")
for file in files {
    print(file)
}
```

### Path Inspection

#### `fs.is_dir(path) -> bool`

Checks if a path is a directory.

**Parameters:**
- `path` - Path to check

**Returns:** `true` if the path is an existing directory, `false` otherwise

```graphoid
if fs.is_dir("src") {
    print("src is a directory")
}
```

#### `fs.is_file(path) -> bool`

Checks if a path is a regular file.

**Parameters:**
- `path` - Path to check

**Returns:** `true` if the path is an existing file, `false` otherwise

```graphoid
if fs.is_file("Cargo.toml") {
    print("Cargo.toml exists")
}
```

#### `fs.exists(path) -> bool`

Checks if a path exists (file or directory).

**Parameters:**
- `path` - Path to check

**Returns:** `true` if the path exists, `false` otherwise

```graphoid
if fs.exists("config.json") {
    print("Config file found")
} else {
    print("Using default configuration")
}
```

## Examples

### Reading a File

```graphoid
import "fs"

handle = fs.open("input.txt", "r")
content = fs.read(handle, 10000)
fs.close(handle)
print(content)
```

### Writing a File

```graphoid
import "fs"

handle = fs.open("output.txt", "w")
fs.write(handle, "Line 1\n")
fs.write(handle, "Line 2\n")
fs.close(handle)
```

### Recursively Finding Files

```graphoid
import "fs"

fn find_gr_files(path) {
    results = []

    if fs.is_file(path) {
        if path.ends_with(".gr") {
            results.append!(path)
        }
    } else if fs.is_dir(path) {
        for entry in fs.list_dir(path) {
            if not entry.starts_with(".") {
                full_path = path + "/" + entry
                for file in find_gr_files(full_path) {
                    results.append!(file)
                }
            }
        }
    }

    return results
}

files = find_gr_files(".")
print("Found " + files.length().to_string() + " .gr files")
```
