# Glang Packaging System Design

A comprehensive package management system for the Glang programming language, enabling distributed development and code reuse.

## Philosophy

The Glang packaging system should embody Glang's core principles:
- **Developer Experience First**: Simple, intuitive commands
- **Data Node Consistency**: Package metadata follows Glang's data structures
- **Functional Programming**: Composable packages and dependencies
- **Type Safety**: Package interface validation
- **Modern by Default**: Built for contemporary development practices

## System Overview

### Package Types
1. **Core Packages**: Built into the language runtime (io, json, math, etc.)
2. **Standard Packages**: Official packages maintained by Glang team
3. **Community Packages**: User-contributed packages from the registry
4. **Local Packages**: Private packages for internal use

### Package Structure

```
package_name/
├── glang.toml              # Package manifest (required)
├── lib/
│   ├── main.gr             # Main package entry point
│   ├── utils.gr            # Supporting modules
│   └── internal/           # Internal-only modules
│       └── helpers.gr
├── test/
│   ├── test_main.gr        # Unit tests
│   └── integration/        # Integration tests
│       └── test_full.gr
├── docs/
│   ├── README.md           # Package documentation
│   ├── API.md              # API reference
│   └── examples/           # Usage examples
│       └── basic_usage.gr
├── samples/                # Sample programs
│   └── demo.gr
└── .glangignore           # Files to ignore during packaging
```

## Package Manifest (glang.toml)

The package manifest uses TOML format for human readability and follows semantic versioning:

```toml
[package]
name = "advanced_datetime"
version = "2.1.4"
description = "Advanced date/time operations with timezone support"
authors = [
    "Alice Developer <alice@example.com>",
    "Bob Contributor <bob@example.com>"
]
license = "MIT"
homepage = "https://github.com/alice/glang-datetime"
repository = "https://github.com/alice/glang-datetime.git"
documentation = "https://docs.glang.dev/packages/advanced_datetime"
readme = "README.md"
keywords = ["datetime", "timezone", "calendar", "business-days"]
categories = ["date-time", "utilities"]

# Supported Glang versions
glang_version = ">=0.9.0, <2.0.0"

# Package entry points
[package.main]
module = "lib/main.gr"
alias = "datetime"  # Default import alias

# Optional additional entry points
[package.entries]
timezone = "lib/timezone.gr"
calendar = "lib/calendar.gr"
business = "lib/business.gr"

[dependencies]
# Standard dependencies
timezone_data = "^3.2.1"        # Compatible with 3.x.x, >= 3.2.1
calendar_systems = "~2.1.0"     # Compatible with 2.1.x
locale_support = ">=1.0.0, <2.0.0"  # Range specification

# Development dependencies (not installed for end users)
[dev-dependencies]
test_framework = "^1.5.0"
benchmark_suite = "^0.3.0"
doc_generator = "^2.0.0"

# Optional dependencies (user can choose)
[optional-dependencies]
high_precision = { package = "decimal_time", version = "^1.0.0", description = "Nanosecond precision" }
astronomy = { package = "astro_calendar", version = "^0.8.0", description = "Astronomical calculations" }

# Build configuration
[build]
exclude = ["test/", "docs/examples/", "*.tmp"]
include = ["lib/", "README.md", "LICENSE"]

# Package metadata for registry
[registry]
publish = true
categories = ["datetime", "utilities"]
documentation_url = "https://docs.glang.dev/packages/advanced_datetime"
```

## Command Line Interface

### glang-package Command Structure

All packaging operations use the `glang-package` command with subcommands:

```bash
# Package Development
glang-package init [package-name]          # Initialize new package
glang-package new [package-name]           # Create new package (alias for init)
glang-package build                        # Build package for distribution
glang-package test                         # Run package tests
glang-package check                        # Validate package structure
glang-package docs                         # Generate documentation
glang-package clean                        # Clean build artifacts

# Dependency Management  
glang-package install [package-name]       # Install package and dependencies
glang-package install                      # Install all dependencies from glang.toml
glang-package uninstall [package-name]     # Remove package
glang-package update [package-name]        # Update specific package
glang-package update                       # Update all packages
glang-package list                         # List installed packages
glang-package deps                         # Show dependency tree

# Registry Operations
glang-package search [query]               # Search package registry
glang-package info [package-name]          # Show package information
glang-package publish                      # Publish package to registry
glang-package unpublish [version]          # Remove version from registry (restricted)
glang-package login                        # Login to package registry
glang-package logout                       # Logout from registry

# Local Operations
glang-package link [path]                  # Link local package for development
glang-package unlink [package-name]        # Remove local package link
glang-package pack                         # Create package tarball
glang-package verify                       # Verify package integrity
```

### Command Examples

```bash
# Create a new datetime package
glang-package init advanced_datetime
cd advanced_datetime

# Install dependencies
glang-package install timezone_data
glang-package install --dev test_framework

# Test the package
glang-package test

# Publish to registry
glang-package build
glang-package publish

# User installs the package
glang-package install advanced_datetime

# Search for date-related packages
glang-package search datetime
glang-package search --category date-time

# Get package information
glang-package info advanced_datetime --verbose
```

## Package Registry

### Registry Structure
- **Central Registry**: `registry.glang.dev` (official)
- **Mirror Support**: For enterprise/offline environments
- **Package Namespaces**: `@organization/package-name` for scoped packages
- **Version Management**: Semantic versioning with pre-release support

### Package Installation Paths
```
~/.glang/packages/          # User-installed packages
  ├── advanced_datetime/
  │   ├── 2.1.4/           # Version-specific installation
  │   │   ├── lib/
  │   │   └── glang.toml
  │   └── current -> 2.1.4  # Symlink to active version
  └── timezone_data/
      └── 3.2.1/

/usr/local/glang/packages/  # System-wide packages (optional)
./glang_packages/          # Project-local packages
```

## Import System Integration

### Basic Import
```glang
# Install: glang-package install advanced_datetime
import "advanced_datetime"          # Uses default alias from manifest
import "advanced_datetime" as dt    # Custom alias

# Multi-entry packages
import "advanced_datetime/timezone" as tz
import "advanced_datetime/business" as biz
```

### Package Resolution
1. **Project-local packages** (`./glang_packages/`)
2. **User packages** (`~/.glang/packages/`)
3. **System packages** (`/usr/local/glang/packages/`)
4. **Core packages** (built into runtime)

### Version Resolution
```glang
# Specific version (not recommended for regular use)
import "advanced_datetime@2.1.4"

# Version ranges in glang.toml handle compatibility
```

## Development Workflow

### Creating a Package

1. **Initialize Package**:
```bash
glang-package init my_utilities
cd my_utilities
```

2. **Package Structure Created**:
```
my_utilities/
├── glang.toml              # Basic manifest
├── lib/
│   └── main.gr             # Entry point with module declaration
├── test/
│   └── test_main.gr        # Basic test structure
└── README.md               # Documentation template
```

3. **Initial lib/main.gr**:
```glang
module my_utilities
alias utils

# Export public functions and data
func hello_world() {
    return "Hello from my_utilities package!"
}

num version = 1.0
```

4. **Develop and Test**:
```bash
# Add dependencies
glang-package install some_dependency

# Run tests
glang-package test

# Check package validity
glang-package check
```

5. **Publish**:
```bash
glang-package build
glang-package publish
```

### Using a Package

1. **Install**:
```bash
glang-package install my_utilities
```

2. **Import and Use**:
```glang
import "my_utilities"

message = utils.hello_world()
print(message)  # "Hello from my_utilities package!"
print("Package version: " + utils.version.to_string())
```

## Dependency Management

### Semantic Versioning
- **Major.Minor.Patch** (e.g., `2.1.4`)
- **Pre-release**: `2.1.4-beta.1`, `2.1.4-rc.2`
- **Build metadata**: `2.1.4+build.123`

### Version Constraints
```toml
[dependencies]
exact = "=1.2.3"           # Exactly version 1.2.3
caret = "^1.2.3"           # >=1.2.3, <2.0.0 (compatible)
tilde = "~1.2.3"           # >=1.2.3, <1.3.0 (reasonably close)
range = ">=1.0.0, <2.0.0"  # Explicit range
wildcard = "1.2.*"         # 1.2.0 <= version < 1.3.0
```

### Dependency Resolution Algorithm
1. **Parse constraints** from all packages
2. **Build dependency graph** with version requirements
3. **Resolve conflicts** using highest compatible versions
4. **Detect circular dependencies**
5. **Install in topological order**

## Security and Trust

### Package Signing
- **Cryptographic signatures** for published packages
- **Publisher verification** through registry accounts
- **Integrity checks** during installation

### Security Scanning
```bash
glang-package audit                    # Check for security vulnerabilities
glang-package audit --fix              # Update vulnerable dependencies
glang-package verify [package-name]    # Verify package integrity
```

### Trust Model
- **Registry trust**: Official packages are verified
- **Publisher trust**: Track record and reputation
- **Community trust**: Download statistics and reviews
- **Code review**: Open source packages can be inspected

## Advanced Features

### Private Registries
```toml
# In glang.toml or user config
[registries]
default = "https://registry.glang.dev"
enterprise = "https://packages.mycompany.com"
```

```bash
glang-package install --registry enterprise internal_package
```

### Package Templates
```bash
# Create packages from templates
glang-package init --template web_service my_api
glang-package init --template cli_tool my_tool
glang-package init --template library my_lib
```

### Build Scripts
```toml
# In glang.toml
[build]
pre_build = "scripts/generate_constants.gr"
post_build = "scripts/optimize.gr"
```

### Local Development
```bash
# Link local package for development
cd ~/my_package
glang-package link

cd ~/other_project
glang-package install my_package  # Uses local linked version
```

## Integration with IDE and Tools

### VS Code Extension
- **Package explorer** showing installed packages
- **Dependency visualization**
- **Package search and install** from editor
- **Auto-import** suggestions from installed packages

### Debugging Support
- **Source maps** for packages
- **Stack trace** integration across package boundaries
- **Package version** information in debug output

## Future Considerations

### Phase Integration
- **Phase 2 (Graph Foundation)**: Packages containing graph algorithms
- **Phase 3 (Self-Aware Systems)**: Packages with self-modifying capabilities
- **Phase 4 (Distributed)**: Packages for distributed computing

### Package Categories by Phase
```toml
[package]
phase_support = ["foundation", "graph", "self-aware"]  # Which phases this package supports
```

### Advanced Package Types
- **Binary packages**: Compiled extensions
- **Template packages**: Code generators
- **Plugin packages**: Runtime extensions
- **Graph packages**: Specialized for graph operations

## Migration and Compatibility

### From Built-in Modules
```glang
# Current (built-in)
import "json"

# Future (packaged, but seamless)
import "json"  # Still works, now resolved through package system
```

### Version Migration
- **Automated migration** tools for breaking changes
- **Deprecation warnings** with migration guidance
- **Compatibility shims** for major version transitions

## Implementation Phases

### Phase 1: Basic Package System (Q2 2025)
- Core `glang-package` commands
- Basic registry implementation
- Simple dependency resolution
- Integration with existing import system

### Phase 2: Advanced Features (Q3 2025)
- Security and signing
- Private registries
- Build scripts and templates
- IDE integration

### Phase 3: Ecosystem Growth (Q4 2025)
- Community registry
- Package discovery and recommendations
- Advanced dependency management
- Performance optimizations

---

This packaging system will enable Glang to grow beyond a single-developer language into a thriving ecosystem where the community can contribute specialized packages for different domains, following the successful models of RubyGems, npm, and Cargo.