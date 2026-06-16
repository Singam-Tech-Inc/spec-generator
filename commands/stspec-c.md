---
name: stspec-c
description: Generate API specification files with auto-numbered specs, structured task directories, and git integration. Triggered by queries like "create a spec", "generate spec", "new API specification".
arguments:
  - name: description
    description: Specification description (required)
    required: true
  - name: title
    description: Specification title (optional, auto-generated from description if not provided)
    required: false
  - name: template
    description: Custom template name from .claude/spec-templates/
    required: false
allowed-tools:
  - Bash
---

Generate API specification with the following workflow:

1. **Parse Arguments**: Accept `description` (required), optional `title` and `template`
2. **Invoke Binary**: Call the stspec Rust binary with provided arguments
3. **Create Structure**: Binary creates:
   - `spec/SPEC-NNNNN-<title>.md` - Specification file
   - `spec/SPEC-NNNNN/XX-task-name.md` - Taskfiles with story points
4. **Git Integration**: Automatically creates branch and stages files
5. **Output**: Display created file paths and next steps

## Usage

```bash
/stspec:c --description "User authentication API with OAuth2 support"
```

With custom title:
```bash
/stspec:c --description "..." --title "Auth API"
```

With custom template:
```bash
/stspec:c --description "..." --template rest-api
```

## Workflow

The command:
1. Validates that a git repository exists (searches upward from current directory)
2. Finds the next available spec number (auto-increment)
3. Creates specification directory structure
4. Generates spec file from template with Jekyll frontmatter
5. Creates taskfiles with story points (Fibonacci: 1, 2, 3, 5, 8, 13, 21)
6. Creates git branch: `spec-NNNNN-<title>`
7. Stages all files for commit
8. Shows confirmation with file paths and next steps

## Implementation

Execute the stspec binary from the plugin's scripts directory, passing all arguments through.

The binary location: `${CLAUDE_PLUGIN_ROOT}/skills/stspec/scripts/stspec`

Command to execute:
```bash
"${CLAUDE_PLUGIN_ROOT}/skills/stspec/scripts/stspec" \
  --description "$DESCRIPTION" \
  ${TITLE:+--title "$TITLE"} \
  ${TEMPLATE:+--template "$TEMPLATE"}
```

## Output

Returns the spec generation output showing:
- Created specification directory
- Generated spec file path
- Taskfiles directory location
- Git branch name created
- Files staged for commit
- Next steps for user

## Notes

- Specification files use Jekyll YAML frontmatter
- Taskfiles are auto-generated based on template and description
- Git branch is local only (user pushes manually)
- Project root detected from `.git/` or `.claude/settings.json`
