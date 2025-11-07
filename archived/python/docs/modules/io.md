# IO Module

The IO module provides input/output operations for file system interaction, console I/O, path manipulation, and network operations.

## Importing

```glang
import "io"
```

## Console Operations

### print(value, newline=true)
Prints a value to the console. By default adds a newline.

```glang
io.print("Hello, World!")  # Prints with newline
io.print(42)               # Prints number with newline
io.print([1, 2, 3])        # Prints list with newline

# Print without newline
io.print("Enter name: ", false)  # No newline
io.print("Processing...", false)  # Continue on same line
```

### input(prompt="")
Reads a line of input from the user, optionally displaying a prompt.

```glang
# Simple input
name = io.input()

# Input with prompt
name = io.input("Enter your name: ")
age_str = io.input("Enter your age: ")
age = age_str.to_num()

# Multi-line input collection
lines = []
print("Enter text (empty line to finish):")
while true {
    line = io.input()
    if line == "" {
        break
    }
    lines.append(line)
}
```

## File Handle Operations

For incremental file I/O operations, use file handles that provide controlled access to files:

### open(filepath, mode)
Creates a file handle capability for incremental I/O. Returns a file handle object.

**Modes:**
- `"r"` - Read-only capability (auto-closes on EOF)
- `"w"` - Write-only capability (manual close required)
- `"a"` - Append-only capability (manual close required)

```glang
import "io"

# Read handle - auto-closes when EOF reached
read_handle = io.open("data.txt", "r")
content = read_handle.read()  # Reads entire file, auto-closes
# read_handle.read()          # Error: capability exhausted

# Write handle - manual control
write_handle = io.open("output.txt", "w")
write_handle.write("Line 1\n")
write_handle.write("Line 2\n")
write_handle.close()  # Manual close required

# Append handle
append_handle = io.open("log.txt", "a")
append_handle.write("New log entry\n")
append_handle.close()
```

### File Handle Methods

**Read Capabilities (auto-close on EOF):**
- `read()` - Reads entire file content, auto-closes on completion
- `read_line()` - Reads next line, auto-closes when EOF reached
- `close()` - Manually close (usually not needed for reads)
- `capability_type()` - Returns `"read"`

**Write/Append Capabilities (manual control):**
- `write(content)` - Writes content to file
- `flush()` - Flushes write buffers to disk
- `close()` - Closes file (can be reopened for more writing)
- `capability_type()` - Returns `"write"` or `"append"`

**All Capabilities:**
- `kill()` - Permanently destroys capability (cannot be reopened)

```glang
# Incremental file processing
input_handle = io.open("large_file.txt", "r")
output_handle = io.open("processed.txt", "w")

# Process line by line (read handle auto-closes at EOF)
while true {
    line = input_handle.read_line()
    if line == "" {  # EOF reached, handle auto-closed
        break
    }
    processed_line = line.upper()
    output_handle.write(processed_line + "\n")
}

output_handle.close()  # Must manually close write handles
```

### File Handle Lifecycle

**Read Capabilities** - Auto-close and exhaustion:
```glang
handle = io.open("file.txt", "r")
content = handle.read()    # Auto-closes on EOF
# handle.read()            # Error: "EOF reached, capability exhausted"
```

**Write Capabilities** - Manual control:
```glang
handle = io.open("file.txt", "w") 
handle.write("First batch")
handle.close()
handle.write("Second batch")  # Reopens file, overwrites content
handle.close()
```

**Permanent Destruction:**
```glang
handle = io.open("file.txt", "w")
handle.write("content")
handle.kill()              # Permanently destroyed
# handle.write("more")     # Error: cannot use killed capability
```

## File Reading Operations

### read_file(path)
Reads the entire contents of a file as a string.

```glang
# Read text file
content = io.read_file("data.txt")
print("File contents: " + content)

# Process file content
config_text = io.read_file("config.txt")
lines = config_text.split("\n")
for line in lines {
    print("Line: " + line)
}
```

### read_lines(path)
Reads a file and returns its contents as a list of lines.

```glang
# Read file as lines
lines = io.read_lines("data.txt")
for line in lines {
    print("Processing: " + line.trim())
}

# Filter empty lines
non_empty = lines.filter("non_empty")
```

### read_binary(path)
Reads a file as binary data, returning a list of byte values (0-255).

```glang
# Read binary file
bytes = io.read_binary("image.png")
print("File size: " + bytes.size().to_string() + " bytes")

# Check file signature
if bytes[0] == 137 and bytes[1] == 80 {
    print("PNG file detected")
}
```

## File Writing Operations

### write_file(path, content)
Writes content to a file, overwriting if it exists. Returns true on success.

```glang
# Write text to file
content = "Hello, World!\nLine 2\nLine 3"
success = io.write_file("output.txt", content)
if success {
    print("File written successfully")
}

# Save configuration
config = "timeout=30\nretries=3\ndebug=true"
io.write_file("config.ini", config)
```

### write_lines(path, lines)
Writes a list of strings as lines to a file. Returns true on success.

```glang
# Write lines to file
lines = ["First line", "Second line", "Third line"]
success = io.write_lines("output.txt", lines)

# Process and save data
data = ["apple", "banana", "cherry"]
processed = data.map("upper")
io.write_lines("processed.txt", processed)
```

### write_binary(path, bytes)
Writes binary data (list of byte values) to a file. Returns true on success.

```glang
# Write binary data
bytes = [137, 80, 78, 71, 13, 10, 26, 10]  # PNG header
success = io.write_binary("output.bin", bytes)

# Copy binary file
data = io.read_binary("input.bin")
io.write_binary("copy.bin", data)
```

### append_file(path, content)
Appends content to an existing file. Returns true on success.

```glang
# Append to log file
log_entry = "[2024-01-15] Application started\n"
io.append_file("app.log", log_entry)

# Add multiple entries
for i in [1, 2, 3] {
    entry = "Entry " + i.to_string() + "\n"
    io.append_file("data.log", entry)
}
```

## File System Operations

### exists(path)
Checks if a file or directory exists.

```glang
if io.exists("config.txt") {
    config = io.read_file("config.txt")
} else {
    print("Config file not found, using defaults")
}

# Check before writing
if not io.exists("output.txt") {
    io.write_file("output.txt", "Initial content")
}
```

### is_file(path)
Checks if a path points to a regular file.

```glang
if io.is_file("data.txt") {
    content = io.read_file("data.txt")
} else {
    print("Not a regular file")
}
```

### is_dir(path)
Checks if a path points to a directory.

```glang
if io.is_dir("logs") {
    files = io.list_dir("logs")
    print("Log files: " + files.to_string())
} else {
    io.make_dir("logs")
}
```

### file_size(path)
Returns the size of a file in bytes.

```glang
size = io.file_size("data.txt")
print("File size: " + size.to_string() + " bytes")

if size > 1048576 {  # 1 MB
    print("Warning: Large file")
}
```

### list_dir(path)
Lists the contents of a directory.

```glang
# List current directory
files = io.list_dir(".")
for file in files {
    print("Found: " + file)
}

# List specific directory
if io.exists("data") {
    data_files = io.list_dir("data")
    txt_files = data_files.filter(f => f.contains(".txt"))
    print("Text files: " + txt_files.to_string())
}
```

### make_dir(path)
Creates a directory. Returns true on success.

```glang
# Create single directory
success = io.make_dir("output")
if success {
    print("Directory created")
}

# Create nested structure
io.make_dir("data")
io.make_dir("data/processed")
io.make_dir("data/raw")
```

### remove_file(path)
Removes a file. Returns true on success.

```glang
# Remove temporary file
if io.exists("temp.txt") {
    success = io.remove_file("temp.txt")
    if success {
        print("Temporary file removed")
    }
}

# Clean up old files
old_files = ["cache1.tmp", "cache2.tmp", "cache3.tmp"]
for file in old_files {
    io.remove_file(file)
}
```

### remove_dir(path)
Removes an empty directory. Returns true on success.

```glang
# Remove empty directory
if io.is_dir("temp") {
    success = io.remove_dir("temp")
    if not success {
        print("Directory might not be empty")
    }
}
```

## Working Directory

### get_cwd()
Returns the current working directory.

```glang
current_dir = io.get_cwd()
print("Working directory: " + current_dir)
```

### set_cwd(path)
Changes the current working directory. Returns true on success.

```glang
# Change to specific directory
success = io.set_cwd("/home/user/projects")
if success {
    print("Changed to: " + io.get_cwd())
}

# Temporarily change directory
original = io.get_cwd()
io.set_cwd("data")
# ... do work in data directory
io.set_cwd(original)  # Return to original
```

## Path Manipulation

### join_path(components...)
Joins path components into a single path string.

```glang
# Join path components
path = io.join_path("home", "user", "documents", "file.txt")
# Result: "home/user/documents/file.txt"

# Build dynamic paths
base = "/var/log"
filename = "app.log"
full_path = io.join_path(base, filename)
```

### split_path(path)
Splits a path into directory and filename components.

```glang
components = io.split_path("/home/user/file.txt")
# Returns ["/home/user", "file.txt"]

dir = components[0]
file = components[1]
print("Directory: " + dir)
print("Filename: " + file)
```

### basename(path)
Returns the filename component of a path.

```glang
filename = io.basename("/home/user/documents/report.pdf")
# Returns "report.pdf"

name = io.basename("data/processed/results.csv")
# Returns "results.csv"
```

### dirname(path)
Returns the directory component of a path.

```glang
directory = io.dirname("/home/user/documents/report.pdf")
# Returns "/home/user/documents"

dir = io.dirname("data/processed/results.csv")
# Returns "data/processed"
```

### extension(path)
Returns the file extension (including the dot).

```glang
ext = io.extension("document.pdf")
# Returns ".pdf"

ext = io.extension("archive.tar.gz")
# Returns ".gz"

ext = io.extension("README")
# Returns "" (no extension)
```

### resolve_path(path)
Resolves a path to its absolute form.

```glang
# Resolve relative path
abs_path = io.resolve_path("../data/file.txt")
print("Absolute path: " + abs_path)

# Resolve current directory
current = io.resolve_path(".")
print("Current absolute: " + current)
```

## Network Operations *(Added in v0.5)*

### http_get(url)
Makes an HTTP GET request to the specified URL and returns the response body as a string.

```glang
# Simple GET request
response = io.http_get("https://api.example.com/data")
print("Response: " + response)

# Get JSON data
json_response = io.http_get("https://jsonplaceholder.typicode.com/users/1")
user_data = json.decode(json_response)
print("User name: " + user_data.get("name").value())

# Check API status
try {
    status = io.http_get("https://api.service.com/health")
    print("Service is up: " + status)
} catch (error) {
    print("Service unavailable: " + error.to_string())
}
```

### http_post(url, data)
Makes an HTTP POST request with optional data and returns the response body as a string.

```glang
# POST with form data
response = io.http_post("https://httpbin.org/post", "key=value&name=test")
print("Response: " + response)

# POST without data
response = io.http_post("https://api.example.com/trigger")
print("Triggered successfully")

# Submit form data
form_data = "username=alice&password=secret123"
login_response = io.http_post("https://example.com/login", form_data)
print("Login response: " + login_response)

# API data submission
api_data = "action=update&id=123&status=active"
result = io.http_post("https://api.service.com/update", api_data)
```

### download_file(url, filepath)
Downloads a file from a URL and saves it to the local filesystem. Returns true on success.

```glang
# Download a file
success = io.download_file("https://example.com/data.txt", "downloaded_data.txt")
if success {
    print("File downloaded successfully")
    content = io.read_file("downloaded_data.txt")
    print("Content: " + content)
}

# Download to specific directory
io.make_dir("downloads")
url = "https://github.com/user/repo/archive/main.zip"
io.download_file(url, "downloads/repo.zip")

# Download with error handling
try {
    success = io.download_file("https://api.service.com/export", "export.csv")
    if success {
        lines = io.read_lines("export.csv")
        print("Downloaded " + lines.size().to_string() + " lines")
    }
} catch (error) {
    print("Download failed: " + error.to_string())
}
```

### send_email(to_addr, subject, body, smtp_server)
Sends an email notification. Currently returns an error indicating this feature is not yet implemented.

```glang
# This will currently fail with "not yet implemented" error
try {
    io.send_email("user@example.com", "Alert", "System status update")
} catch (error) {
    print("Email not available: " + error.to_string())
}
```

## Examples

### File Processing Pipeline
```glang
import "io"

# Read and process a CSV file
if io.exists("data.csv") {
    lines = io.read_lines("data.csv")
    header = lines[0]
    data_lines = lines[1:]  # Skip header
    
    processed = []
    for line in data_lines {
        fields = line.split(",")
        # Process fields...
        processed.append(fields.join("|"))
    }
    
    io.write_lines("processed.csv", processed)
    print("Processed " + data_lines.size().to_string() + " records")
}
```

### Directory Scanner
```glang
import "io"

func scan_directory(path, extension) {
    if not io.is_dir(path) {
        return []
    }
    
    files = io.list_dir(path)
    matching = []
    
    for file in files {
        full_path = io.join_path(path, file)
        if io.is_file(full_path) {
            if io.extension(file) == extension {
                matching.append(full_path)
            }
        }
    }
    
    return matching
}

# Find all .txt files
txt_files = scan_directory("documents", ".txt")
for file in txt_files {
    print("Found: " + file)
}
```

### Log File Manager
```glang
import "io"

# Rotate log files
current_log = "app.log"
if io.exists(current_log) {
    size = io.file_size(current_log)
    if size > 1048576 {  # 1 MB
        # Archive current log
        timestamp = "20240115_120000"  # Would use real timestamp
        archive_name = "app_" + timestamp + ".log"
        
        # Read current log
        content = io.read_file(current_log)
        
        # Write to archive
        io.write_file(archive_name, content)
        
        # Clear current log
        io.write_file(current_log, "")
        io.append_file(current_log, "Log rotated at " + timestamp + "\n")
    }
}
```

### Configuration Manager
```glang
import "io"

func load_config(filename) {
    config = {}
    
    if not io.exists(filename) {
        print("Config file not found: " + filename)
        return config
    }
    
    lines = io.read_lines(filename)
    for line in lines {
        line = line.trim()
        if line != "" and not line.contains("#") {  # Skip comments
            parts = line.split("=")
            if parts.size() == 2 {
                key = parts[0].trim()
                value = parts[1].trim()
                config[key] = value
            }
        }
    }
    
    return config
}

func save_config(filename, config) {
    lines = []
    for key in config.keys() {
        line = key + "=" + config[key].value()
        lines.append(line)
    }
    
    return io.write_lines(filename, lines)
}

# Use configuration
config = load_config("app.conf")
config["debug"] = "true"
save_config("app.conf", config)
```

### Binary File Operations
```glang
import "io"

# Read and analyze binary file
bytes = io.read_binary("data.bin")

# Check file signature/magic number
if bytes.size() >= 4 {
    magic = bytes[0:4]
    print("Magic number: " + magic.to_string())
}

# Modify binary data
for i in [0, 1, 2, 3] {
    if i < bytes.size() {
        bytes[i] = 0  # Clear first 4 bytes
    }
}

# Write modified data
io.write_binary("modified.bin", bytes)
```

### Web Service Integration
```glang
import "io"
import "json"

func fetch_user_data(user_id) {
    # Fetch user data from API
    url = "https://jsonplaceholder.typicode.com/users/" + user_id.to_string()
    
    try {
        response = io.http_get(url)
        user_data = json.decode(response)
        return user_data
    } catch (error) {
        print("Failed to fetch user: " + error.to_string())
        return {}
    }
}

func save_user_report(user_data) {
    # Create user report
    name = user_data.get("name").value()
    email = user_data.get("email").value()
    
    report_lines = [
        "User Report",
        "===========",
        "Name: " + name,
        "Email: " + email,
        "Generated: " + "2025-01-11"  # Would use real timestamp
    ]
    
    filename = name.replace(" ", "_").lower() + "_report.txt"
    return io.write_lines(filename, report_lines)
}

# Process multiple users
user_ids = [1, 2, 3, 4, 5]
for user_id in user_ids {
    user_data = fetch_user_data(user_id)
    if user_data.size() > 0 {
        success = save_user_report(user_data)
        if success {
            print("Report saved for user " + user_id.to_string())
        }
    }
}
```

### File Backup System
```glang
import "io"

func backup_to_remote(local_file, backup_url) {
    # Read local file
    if not io.exists(local_file) {
        print("Local file not found: " + local_file)
        return false
    }
    
    content = io.read_file(local_file)
    
    # Upload to backup service (simplified example)
    try {
        response = io.http_post(backup_url, content)
        print("Backup successful: " + response)
        return true
    } catch (error) {
        print("Backup failed: " + error.to_string())
        return false
    }
}

func download_backup(backup_url, local_file) {
    # Download backup file
    try {
        success = io.download_file(backup_url, local_file)
        if success {
            print("Backup downloaded to: " + local_file)
            return true
        } else {
            print("Download failed")
            return false
        }
    } catch (error) {
        print("Download error: " + error.to_string())
        return false
    }
}

# Example usage
files_to_backup = ["config.txt", "data.csv", "app.log"]
for file in files_to_backup {
    backup_url = "https://backup.service.com/upload/" + file
    backup_to_remote(file, backup_url)
}
```