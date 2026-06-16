# Spec Generator Plugin

Automatically generate API specification files with structured directories, task breakdowns, and git integration.

## Features

- **Auto-numbered specs** — SPEC-NNNNN format with auto-increment
- **Structured layout** — Organized directories with taskfiles for each spec
- **Git integration** — Creates branches, manages commits, integrates with workflow
- **Template support** — Default API spec template + project-defined custom templates
- **Intelligent task generation** — Claude AI generates subtasks from spec description
- **User prompts** — Fine-tune generated content via interactive feedback
- **Jekyll YAML format** — Templates use Jekyll format with story points (Fibonacci)

## Installation

1. Copy this directory to your Claude Code plugins folder or reference it locally
2. Enable the plugin in Claude Code settings
3. Run `/help` to verify the plugin loads

## Quick Start

### Basic Usage

```bash
/stspec:c --description "User authentication API with OAuth2 support"
```

This will:
1. Create a new spec directory: `spec/SPEC-00001-user-authentication/`
2. Generate the API specification file
3. Create taskfiles with auto-generated subtasks
4. Create a git branch: `spec-00001-user-authentication`
5. Ask for confirmation before committing

### With Custom Title

```bash
/stspec:c --description "..." --title "Auth API"
```

### With Custom Template

```bash
/stspec:c --description "..." --template custom-template
```

## Configuration

### Project Root

The plugin reads `project_root` from `.claude/settings.json`:

```json
{
  "project_root": "/path/to/your/repo"
}
```

If not set, the plugin searches upward from the current directory to find `.git/`.

### Custom Templates

Store custom templates in `.claude/spec-templates/`:

```
.claude/
└── spec-templates/
    ├── microservice-template.md
    ├── rest-api-template.md
    └── graphql-template.md
```

Templates use Jekyll format:

```markdown
---
title: API Specification
version: 1.0.0
author: "{{ user }}"
---

## Endpoints

{{ endpoints_section }}

## Request/Response

{{ request_response_section }}
```

## File Structure

After running the command, you'll see:

```
spec/
└── SPEC-00001-user-authentication/
    ├── SPEC-00001-user-authentication.md  # Main spec file
    └── taskfiles/
        ├── 01-subfeature-oauth-flow.md
        ├── 02-test-plan.md
        ├── 03-implementation-endpoints.md
        └── 04-documentation.md
```

## Spec File Format

Specs include:

- API endpoints and methods
- Request/response examples
- Error codes and status codes
- Authentication/authorization
- Rate limiting
- Version information
- Other relevant sections

## Taskfiles

Taskfiles are auto-generated based on:

- The spec description you provide
- The template structure
- Claude AI analysis to break down into logical subtasks

Each taskfile includes:
- **Story points** (Fibonacci series: 1, 2, 3, 5, 8, 13, 21...)
- **Sub-feature** or **sub-task** description
- **Acceptance criteria**
- **Implementation notes**

### Refining Taskfiles

After generation, you can:

1. **Edit directly** — Modify the `.md` files in `taskfiles/`
2. **Provide feedback** — Ask Claude to refine using prompts like:
   - "Add more detail to the test plan"
   - "Split the implementation task into smaller pieces"
   - "Add acceptance criteria"

## Git Workflow

The plugin creates and manages git branches:

1. **Branch creation** — Creates `spec-NNNNN-shorttitle`
2. **File creation** — Generates spec and taskfiles
3. **Confirmation** — Shows created files, asks to update/confirm
4. **Commit** — Commits to the branch with message: `"Add SPEC-NNNNN: <title>"`
5. **Local only** — Does not push; merge to main manually

## Troubleshooting

### "Git repo not found"

Ensure you're in a git repository or set `project_root` in `.claude/settings.json`.

### "Template not found"

Check that custom templates are in `.claude/spec-templates/` with correct filename.

### "Spec number conflict"

The plugin auto-increments to avoid conflicts. If issues persist, manually check `spec/` directory for existing specs.

## Next Steps

- Create your first spec: `/stspec:c --description "..."`
- Customize templates in `.claude/spec-templates/`
- Fine-tune generated taskfiles
- Merge branches to main when specs are complete

## Support

For issues, check the plugin logs or contact the author.

---

**Plugin Version**: 0.1.0  
**Author**: Vairam  
**License**: MIT
