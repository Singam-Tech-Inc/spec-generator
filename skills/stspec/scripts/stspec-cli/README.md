# stspec-cli

Rust CLI tool for generating API specifications with auto-numbering, templating, and git integration.

## Building

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git

### Build for Ubuntu (Release)

```bash
# Navigate to the project directory
cd skills/stspec/scripts/stspec-cli

# Build release binary (optimized, stripped)
cargo build --release

# Binary location: target/release/stspec
```

### Build for Other Platforms

```bash
# macOS
cargo build --release --target x86_64-apple-darwin

# Linux (generic)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Installation

After building, copy the binary to the plugin:

```bash
# Copy to plugin scripts directory
cp target/release/stspec ../../../scripts/stspec

# Or make it executable and add to PATH
chmod +x target/release/stspec
```

## Usage

```bash
# Create spec with description (title auto-generated)
./stspec --description "User authentication API with OAuth2 support"

# Create spec with custom title
./stspec --description "..." --title "Auth API"

# Use custom template
./stspec --description "..." --template rest-api

# Specify project root
./stspec --description "..." --project-root /path/to/repo

# Dry-run (don't create files or git branch)
./stspec --description "..." --dry-run
```

## Command Line Arguments

- `--description <TEXT>` (required) - Specification description
- `--title <TEXT>` (optional) - Specification title (max 50 chars). Auto-generated from description if omitted
- `--template <NAME>` (optional) - Custom template name from `.claude/spec-templates/`
- `--project-root <PATH>` (optional) - Override project root path
- `--dry-run` - Don't create files or make git changes

## Project Root Detection

The tool finds the project root in this order:

1. `--project-root` argument if provided
2. `project_root` field in `.claude/settings.json`
3. Search upward from current directory until `.git/` is found

## Features

- **Auto-incrementing spec numbers** - SPEC-NNNNN format
- **Directory structure** - Creates `spec/SPEC-NNNNN-<title>/` with taskfiles
- **Template support** - Custom templates from `.claude/spec-templates/`
- **Git integration** - Creates branches and stages files
- **Placeholder rendering** - Fills template variables (SPEC_ID, TITLE, etc.)
- **Title sanitization** - Replaces special characters with hyphens

## Development

### Running Tests

```bash
cargo test
```

### Code Structure

- `src/main.rs` - CLI entry point and argument parsing
- `src/spec.rs` - Specification generation logic
- `src/template.rs` - Template loading and rendering
- `src/git.rs` - Git operations

### Dependencies

- **clap** - Command-line argument parsing
- **serde_json** - JSON handling
- **regex** - Pattern matching
- **chrono** - Date/time
- **handlebars** - Template rendering (optional, for future enhancement)
- **git2** - Git operations
- **walkdir** - Directory traversal

## Output

The tool creates:

```
spec/
└── SPEC-NNNNN-shorttitle/
    ├── SPEC-NNNNN-shorttitle.md       # Main spec file
    └── taskfiles/
        ├── 01-features.md             # Feature tasks
        ├── 02-test-plan.md            # Test plan
        ├── 03-implementation.md       # Implementation tasks
        └── 04-documentation.md        # Documentation tasks
```

## Git Integration

1. Creates branch: `spec-NNNNN-<title>`
2. Stages all created files
3. Ready for user to commit with custom message

## Error Handling

The tool provides clear error messages for:

- Git repository not found
- Template file not found
- Invalid file paths
- Permission issues
- Invalid arguments

## Future Enhancements

- Claude AI-assisted taskfile generation
- Template inheritance and composition
- Validation of spec structure
- Export to OpenAPI/AsyncAPI formats
- CI/CD integration

## License

MIT

---

**Built with Rust 🦀**
