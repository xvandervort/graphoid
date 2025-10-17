# Enhanced Binary Data Handling Plan

**Status:** Planning Phase  
**Priority:** Medium (after networking and crypto foundation)  
**Target:** Post-Q2 2025

## Overview

This document outlines enhancements to Glang's binary data handling capabilities, building upon the current foundation of `list<num>` for binary data and adding specialized features for complex binary processing, image manipulation, and format detection.

## Current State

### What Works Well
- **Basic Binary I/O**: `io.read_binary()` and `io.write_binary()` handle raw binary data as `list<num>`
- **Path Manipulation**: Complete set of path utilities for file handling
- **JSON Support**: Full JSON encoding/decoding for structured data
- **Type Safety**: `list<num>` provides type-safe binary data representation
- **Existing Operations**: All list operations work on binary data (filter, map, slice, etc.)

### Current Limitations
- **No Hexadecimal Literals**: Cannot write `[0xFF, 0x00, 0xAB]` - must use `[255, 0, 171]`
- **No Fixed-Size Constraints**: Cannot enforce pixel = exactly 3 values
- **No Format Recognition**: Binary data is just "a list of numbers"
- **No Structured Binary Types**: Images, audio, packets all treated as flat byte arrays
- **No Binary Utilities**: No endianness conversion, bit manipulation, hex conversion

## Enhancement Phases

### Phase 1: Numeric Literal Extensions (High Priority)

**Goal**: Support hexadecimal and binary literals in the language

```glang
# Hexadecimal literals
packet_header = [0xFF, 0x00, 0xAB, 0xCD]
crypto_key = [0x2B, 0x7E, 0x15, 0x16, 0x28, 0xAE, 0xD2, 0xA6]

# Binary literals (for bit patterns)
flags = [0b11110000, 0b00001111]
mask = 0b10101010

# Octal literals (for file permissions)
file_mode = 0o755
```

**Implementation**: Extend lexer to recognize hex (`0x`), binary (`0b`), and octal (`0o`) prefixes

### Phase 2: Fixed-Size List Constraints (Medium Priority)

**Goal**: Enforce compile-time or runtime size constraints on lists

```glang
# RGB pixels - exactly 3 values
list<num, 3> rgb_pixel = [255, 128, 64]     # Valid
list<num, 3> bad_pixel = [255, 128]         # ERROR: Wrong size

# RGBA pixels - exactly 4 values  
list<num, 4> rgba_pixel = [255, 128, 64, 255]

# Network packet headers - fixed sizes
list<num, 20> ip_header = [...]             # IPv4 header
list<num, 8> udp_header = [...]             # UDP header

# Matrix dimensions
matrix<num, 480, 640, 3> image_matrix       # Height × Width × Channels
```

**Implementation**: Extend type system to support size constraints on lists

### Phase 3: Binary Utility Functions (Medium Priority)

**Goal**: Essential binary data manipulation operations

```glang
import "binary"

# Endianness conversion
bytes = [0x12, 0x34, 0x56, 0x78]
big_endian = binary.to_uint32(bytes, "big")     # 0x12345678
little_endian = binary.to_uint32(bytes, "little") # 0x78563412
back_to_bytes = binary.from_uint32(0x12345678, "big") # [0x12, 0x34, 0x56, 0x78]

# Bit manipulation
bits = binary.to_bits([0xFF, 0x00])         # [1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]
bytes = binary.from_bits(bits)              # [255, 0]
bit_value = binary.get_bit(255, 7)          # 1 (check bit 7)
new_byte = binary.set_bit(0, 7, 1)          # 128 (set bit 7)

# Format conversion
hex_string = binary.to_hex([255, 171, 205]) # "FFABCD"
bytes = binary.from_hex("FFABCD")           # [255, 171, 205]
base64_str = binary.to_base64([72, 101, 108]) # "SGVs"
bytes = binary.from_base64("SGVs")          # [72, 101, 108]

# Binary arithmetic
checksum = binary.crc32(data)               # CRC32 checksum
xor_result = binary.xor(data1, data2)       # XOR two byte arrays
```

### Phase 4: Format Detection and Parsing (Lower Priority)

**Goal**: Automatic detection and structured parsing of binary formats

```glang
import "formats"

# File format detection
info = formats.detect("unknown_file.bin")
# Returns: {
#   "format": "PNG",
#   "confidence": 0.95,
#   "mime_type": "image/png",
#   "characteristics": {...}
# }

# Format-specific parsing
png_data = formats.parse_png("image.png")
# Returns: {
#   "width": 1920,
#   "height": 1080,
#   "color_type": "RGB",
#   "pixels": [[[r,g,b], [r,g,b], ...], [...]]
# }

# Magic number detection
signatures = {
    "PNG": [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
    "JPEG": [0xFF, 0xD8, 0xFF],
    "GIF": [0x47, 0x49, 0x46, 0x38]
}
detected = formats.detect_by_signature("file.bin")
```

### Phase 5: Image Processing Module (Lower Priority)

**Goal**: Structured image manipulation using Glang's built-in types

```glang
import "image"

# Load image as structured hash
img = image.load("photo.jpg")
# Returns: {
#   "width": 1920,
#   "height": 1080,
#   "format": "JPEG",
#   "pixels": [                    # List of rows
#     [[255, 128, 64], [200, 100, 50], ...],  # Each row = list of RGB pixels
#     [[180, 90, 45], [220, 110, 55], ...]
#   ]
# }

# Pixel manipulation
pixel = img["pixels"][100][200]          # Get pixel [R, G, B]
img["pixels"][100][200] = [0, 255, 0]   # Set pixel to green

# Image transformations (return new image hashes)
resized = image.resize(img, 800, 600)
grayscale = image.to_grayscale(img)
rotated = image.rotate(img, 90)

# Color space conversions
hsv_img = image.rgb_to_hsv(img)
lab_img = image.rgb_to_lab(img)

# Save in different formats
image.save(img, "output.png", format: "PNG")
```

### Phase 6: Advanced Binary Processing (Future)

**Goal**: Specialized binary data structures for different domains

```glang
# Audio data as structured hash
import "audio"
audio_data = audio.load("song.wav")
# Returns: {
#   "sample_rate": 44100,
#   "channels": 2,
#   "samples": [
#     [0.5, -0.3, 0.8, ...],     # Left channel
#     [0.2, -0.1, 0.6, ...]      # Right channel
#   ]
# }

# Network packet parsing
import "network"
packet = network.parse_packet(binary_data, "TCP")
# Returns: {
#   "headers": {...},
#   "payload": [...],
#   "checksum": 0x1234
# }

# Archive handling
import "archive"
archive_info = archive.get_info("data.zip")
files = archive_info["files"]           # List of file info hashes
content = archive.extract_file("data.zip", "readme.txt")
```

## Design Principles

### Use Glang's Built-in Types
- **Lists** for ordered data (pixels, samples, bytes)
- **Hashes** for structured metadata
- **Numbers** for individual values
- **Strings** for text data and encoded representations
- **No custom classes** - everything builds on existing types

### Maintain Backward Compatibility
- Current `io.read_binary()` and `io.write_binary()` remain unchanged
- All existing binary operations continue to work
- New features are additive, not breaking

### Graph-Theoretic Future Compatibility
- Binary data structures designed to eventually become graph nodes
- Pixel relationships could become edges with color properties
- File headers could link to payload data nodes

## Implementation Notes

### Numeric Literals Extension
- Requires lexer changes to recognize `0x`, `0b`, `0o` prefixes
- Should convert to same underlying `num` type
- Provides syntax sugar for binary data constants

### Fixed-Size Lists
- Could be compile-time checking (preferred) or runtime validation
- Syntax: `list<type, size>` extends existing `list<type>` pattern
- Error on size mismatch during creation or modification

### Binary Utilities
- Implemented as pure functions in `binary` module
- Work with existing `list<num>` representations
- No new types needed - just operations on existing data

## Testing Strategy

### Unit Tests
- Hexadecimal literal parsing
- Fixed-size list constraint enforcement
- Binary utility function correctness
- Format detection accuracy

### Integration Tests
- Complete image processing workflows
- Binary format round-trip conversion
- Cross-platform endianness handling

### Performance Tests
- Large binary file processing
- Memory usage with binary data
- Speed of format detection algorithms

## Success Metrics

1. **Developer Productivity**: Can write `[0xFF, 0x00]` instead of `[255, 0]`
2. **Type Safety**: Fixed-size constraints prevent dimension errors
3. **Format Support**: Automatic detection of common binary formats
4. **Processing Power**: Rich image and binary manipulation capabilities
5. **Backward Compatibility**: All existing code continues to work

## Related Work

- Current JSON module provides model for format-specific parsing
- Path manipulation utilities show successful extension of I/O capabilities
- Existing freeze() system demonstrates successful type constraint implementation

---

**Next Steps**: Prioritize hexadecimal literal support as immediate enhancement for crypto/networking work, then implement remaining phases post-networking foundation.