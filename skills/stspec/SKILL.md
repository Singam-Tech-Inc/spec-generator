---
name: Spec Generator
description: Use this skill to generate versioned API specifications with auto-numbered IDs, structured task breakdowns, and git integration. Triggered by queries like "create a spec", "generate spec", "/stspec:c", or "new API specification".
version: 0.1.0
---

## Overview

Generate API specification files with automatic numbering, structured task breakdowns, git integration, and AI-assisted task refinement.

The skill provides an interactive workflow:
1. Accept specification description (required) and optional title
2. Auto-generate SPEC-NNNNN number and directory structure
3. Create specification file from template
4. Generate intelligent taskfiles (subtasks, test plan, implementation)
5. Create git branch and request confirmation before commit
6. Allow user to refine generated taskfiles through prompts or direct editing

## Workflow

### Input Phase

Collect required inputs:
- **Description** (required): Full specification description (can be detailed or brief)
- **Title** (optional): Short title (max 50 chars). If omitted, auto-generate from first 50 characters of description
- **Template** (optional): Specify custom template from `.claude/spec-templates/`. Defaults to built-in template

### Generation Phase

Invoke the spec generation binary with inputs:
- Determine `project_root` from `.claude/settings.json` or search upward for `.git/`
- Find highest existing spec number in `spec/` directory and auto-increment
- Sanitize title: replace special characters with hyphens
- Create directory: `spec/SPEC-NNNNN-<title>/`
- Generate spec file: `spec/SPEC-NNNNN-<title>/SPEC-NNNNN-<title>.md`
- Load selected template and apply to spec file
- Prepare to generate taskfiles

### Taskfile Generation Phase

Analyze spec description and template structure to intelligently generate taskfiles:
- **Claude AI Analysis**: Use spec description + template + skeleton to generate detailed subtasks
- **Content**: Each taskfile includes subtask description, story points (Fibonacci), acceptance criteria
- **Location**: Save as `spec/SPEC-NNNNN/taskfiles/XX-task-name.md` (one file per subtask)
- **Templates-driven**: Task structure based on template; content based on description analysis

Generate default taskfiles including:
- Sub-feature implementation tasks
- Test plan with coverage details
- Documentation requirements
- Additional domain-specific tasks based on API type

### Confirmation Phase

Show user:
```
✓ Created SPEC-00042-user-auth
  - File: spec/SPEC-00042-user-auth/SPEC-00042-user-auth.md
  - Tasks: spec/SPEC-00042-user-auth/taskfiles/
  - Branch: spec-00042-user-auth
  
Generated 6 taskfiles (sub-features, test plan, implementation, docs)
```

Present generated taskfiles and ask:
- "Review the generated taskfiles. Would you like to edit them directly, provide feedback to refine, or proceed?"

### Refinement Phase (Optional)

Allow iterative refinement through two paths:

**Path 1: Direct Editing**
- Show file paths where user can edit directly in their editor
- User modifies `.md` files and provides signal to proceed

**Path 2: Feedback Prompts**
- Accept natural language feedback: "Add more test cases", "Split implementation into smaller tasks"
- Claude refines the taskfiles based on feedback and shows updated versions
- Can repeat until satisfied

### Commit Phase

Create git branch and request final confirmation:
- Branch name: `spec-NNNNN-<title>`
- Ask: "Proceed with commit to branch `spec-00042-user-auth`?"
- If yes: Commit with message: `"Add SPEC-NNNNN: <title>"`
- If no: Leave files created but uncommitted for user to handle manually

## Files and Directories

### Input Files

- **`.claude/settings.json`** (optional): Should contain `project_root` field pointing to repo root
- **`.claude/spec-templates/`** (optional): Custom template files in Jekyll markdown format
  - Example: `.claude/spec-templates/rest-api.md`

### Output Structure

```
spec/
├── SPEC-NNNNN-shorttitle.md           # Main specification file
└── SPEC-NNNNN/
    ├── 01-subfeature-oauth.md         # Auto-generated subtasks
    ├── 02-subfeature-rate-limit.md
    ├── 03-test-plan.md
    ├── 04-implementation.md
    └── 05-documentation.md
```

Each taskfile uses Jekyll frontmatter:
```markdown
---
title: "OAuth2 Flow Implementation"
story_points: 8
status: "pending"
---

## Description
[Detailed task description]

## Acceptance Criteria
- Criterion 1
- Criterion 2
- Criterion 3

## Implementation Notes
[Any relevant notes]
```

### Template System

#### Default Template

The plugin provides a built-in API specification template with sections for:
- Endpoints (HTTP methods, paths, parameters)
- Request/response examples
- Error codes and HTTP status codes
- Authentication and authorization
- Rate limiting and quotas
- API versioning
- Other relevant API sections

Template applies Jekyll YAML frontmatter:
```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
version: "1.0.0"
description: "{{ SPEC_DESCRIPTION }}"
created_date: "{{ CREATED_DATE }}"
---
```

#### Project-Defined Templates

Store custom templates in `.claude/spec-templates/`:

```
.claude/
└── spec-templates/
    ├── rest-api.md           # Template for REST APIs
    ├── graphql-api.md        # Template for GraphQL APIs
    └── microservice-spec.md  # Template for microservices
```

Template format uses Jekyll markdown with placeholders:
- `{{ SPEC_TITLE }}` - Specification title
- `{{ SPEC_ID }}` - SPEC-NNNNN number
- `{{ SPEC_DESCRIPTION }}` - Full description
- `{{ CREATED_DATE }}` - Current date

Select template with: `/stspec:c --description "..." --template rest-api`

## Git Integration

### Branch Creation

- Creates feature branch: `spec-NNNNN-<sanitized-title>`
- Special character replacement: All non-alphanumeric characters become hyphens
- Example: "User Auth API" → `spec-00042-user-auth`

### Commit Process

- Initial commit message: `"Add SPEC-NNNNN: <title>"`
- All spec files and taskfiles included in commit
- Commit happens after user confirmation
- Local commit only (no push to origin)

### Error Handling

- **Git repo not found**: Error message + instruction to initialize git or set `project_root`
- **No write permissions**: Error with diagnosis
- **Spec number conflict**: Auto-increment to next available number
- **Template not found**: Error with list of available templates

## Advanced: Taskfile Refinement

### AI-Assisted Generation

When generating taskfiles:
1. The Rust CLI creates skeleton taskfiles with placeholder structure
2. Claude AI (in the skill framework) analyzes spec description and template
3. Generate intelligent subtasks based on API type (REST, GraphQL, WebSocket)
4. Include testing, documentation, and deployment considerations  
5. Apply story points (Fibonacci: 1, 2, 3, 5, 8, 13, 21) based on complexity

**Note**: Taskfile content generation is handled by Claude AI within the plugin skill, not by the Rust binary. The binary creates the file structure and applies templates; the skill refines content.

### User Feedback Loop

After initial generation, support iterative refinement:

**Example feedback:**
- "Add database migration tasks to implementation"
- "Break down the test plan into unit, integration, and e2e"
- "Add performance testing requirements"

Claude updates taskfiles and shows new versions for review.

## Additional Resources

For detailed information about template creation and task structure, see:

- **`references/default-template.md`** - Complete default API spec template
- **`references/template-guide.md`** - How to create custom templates
- **`references/task-examples.md`** - Example taskfiles from various API types

## Usage Examples

### Basic Specification

```
/stspec:c --description "User authentication API supporting OAuth2, JWT, and session-based auth"
```

Auto-generates title from first 50 chars: "User authentication API supporting OAuth2, JWT,"

### With Custom Title

```
/stspec:c --description "WebSocket real-time notifications system" --title "WebSocket Notifications"
```

Creates: `SPEC-00043-websocket-notifications`

### With Custom Template

```
/stspec:c --description "..." --template graphql-api
```

Uses template from `.claude/spec-templates/graphql-api.md`

## Integration Notes

- The skill invokes a pre-compiled Rust binary (`stspec-cli`) for deterministic file operations
- Rust binary handles: spec numbering, directory creation, file generation, git operations
- Skill handles: user interaction, prompt refinement, Claude AI-assisted taskfile generation
- All file paths use `project_root` from configuration for portability
