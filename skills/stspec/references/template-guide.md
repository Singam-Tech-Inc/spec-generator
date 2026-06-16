# Custom Template Guide

This guide explains how to create and use custom specification templates.

## Overview

Templates control:
1. **Specification structure** - What sections appear in the API spec
2. **Taskfile generation** - What subtasks are created for the spec
3. **Placeholders** - Dynamic values filled in during spec creation

## Creating Custom Templates

### Step 1: Create Template File

Store custom templates in `.claude/spec-templates/`:

```bash
.claude/
└── spec-templates/
    ├── rest-api.md
    ├── graphql-api.md
    └── webhook-api.md
```

### Step 2: Template Format

Templates use Jekyll-style YAML frontmatter with placeholders:

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
version: "1.0.0"
description: "{{ SPEC_DESCRIPTION }}"
created_date: "{{ CREATED_DATE }}"
# Custom fields specific to your template
api_type: "REST"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Endpoints

[Template content here]
```

### Step 3: Available Placeholders

Placeholders are automatically filled during spec creation:

| Placeholder | Value | Example |
|-------------|-------|---------|
| `{{ SPEC_ID }}` | Auto-generated spec number | SPEC-00042 |
| `{{ SPEC_TITLE }}` | Specification title | "User Authentication API" |
| `{{ SPEC_DESCRIPTION }}` | Full specification description | "Manages user auth via OAuth2..." |
| `{{ CREATED_DATE }}` | Current date (ISO 8601) | 2024-01-15T10:30:00Z |
| `{{ USER_NAME }}` | Current user name | John Doe |

## Template Types

### REST API Template

For standard REST APIs:

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
api_type: "REST"
base_path: "/v1"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Base URL

`https://api.example.com/v1`

## Authentication

[Auth details specific to REST]

## Endpoints

[REST endpoint structure]

## Error Handling

[REST error codes]
```

**Associated Taskfiles**:
- Sub-feature tasks for each endpoint group
- REST-specific test plan (request/response validation)
- Implementation tasks (routing, validation, serialization)
- Documentation (OpenAPI spec, endpoint examples)

### GraphQL API Template

For GraphQL APIs:

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
api_type: "GraphQL"
base_url: "https://api.example.com/graphql"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Schema

[GraphQL schema definition]

## Queries

[Query definitions]

## Mutations

[Mutation definitions]

## Subscriptions

[Subscription definitions]

## Error Handling

[GraphQL-specific error handling]
```

**Associated Taskfiles**:
- Schema definition and type validation
- Query/mutation/subscription implementation
- GraphQL-specific testing (introspection, schema validation)
- Performance optimization (N+1 query prevention)

### Microservice Template

For microservice specifications:

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
type: "Microservice"
dependencies: []
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Service Architecture

[Service components]

## Dependencies

[Service dependencies]

## APIs

[Internal and external APIs]

## Data Models

[Domain models]

## Deployment

[Deployment requirements]
```

**Associated Taskfiles**:
- Service contract definition
- Integration test plan
- Deployment/infrastructure tasks
- Monitoring and observability setup

### Event-Driven Template

For event-driven architecture specifications:

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
type: "Event-Driven"
event_broker: "Kafka|RabbitMQ|SNS"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Events

[Event definitions]

## Producers

[Services producing events]

## Consumers

[Services consuming events]

## Data Flow

[Event flow diagram]

## Error Handling

[Dead-letter queues, retries]
```

**Associated Taskfiles**:
- Event schema definition
- Producer implementation
- Consumer implementation
- Dead-letter queue handling

## Taskfile Generation

### Understanding Taskfile Generation

When you create a spec using a template, the system:

1. **Analyzes your description** to understand the API purpose
2. **Uses the template** to determine task categories
3. **Generates intelligent subtasks** based on API type and description
4. **Adds story points** using Fibonacci series (1, 2, 3, 5, 8, 13, 21)

### Default Taskfile Categories

All templates generate these taskfile categories:

#### 1. Sub-Features/Implementation
**Generated taskfiles for distinct API features**

Example for REST API:
- `01-endpoint-authentication.md` - Implement auth endpoints
- `02-endpoint-users.md` - Implement user management endpoints
- `03-endpoint-resources.md` - Implement resource endpoints

Each includes:
- Implementation details
- Dependencies
- Story points (typically 5-13)

#### 2. Test Plan
**Testing strategy and execution**

File: `XX-test-plan.md`

Includes:
- Unit testing strategy
- Integration testing scope
- API contract testing
- Load/performance testing
- Security testing

Story points: 8-21 (depends on API complexity)

#### 3. Implementation
**Core implementation tasks**

File: `XX-implementation.md`

Includes:
- Framework setup
- Request/response handling
- Error handling implementation
- Validation logic
- Database integration (if applicable)

Story points: 13-21

#### 4. Documentation
**API documentation requirements**

File: `XX-documentation.md`

Includes:
- OpenAPI/GraphQL schema documentation
- Usage examples
- Changelog
- Migration guides
- FAQ

Story points: 5-8

## Using Custom Templates

### Select Template When Creating Spec

```bash
/stspec:c --description "GraphQL API for real-time notifications" --template graphql-api
```

### Template Resolution

1. Check `.claude/spec-templates/graphql-api.md` first
2. Check for filename variations (graphql-api.md, graphql.md)
3. Fall back to default template if not found

### Error Handling

If template not found:
```
Error: Template 'graphql-api' not found

Available templates:
- rest-api.md
- graphql-api.md
- webhook-api.md

Create new template at: .claude/spec-templates/graphql-api.md
```

## Template Best Practices

### 1. Clear Section Headings

```markdown
# {{ SPEC_TITLE }}

## Overview
## Architecture
## Endpoints
## Error Handling
## Deployment
```

### 2. Consistent Formatting

- Use Markdown standards
- Keep line length readable (80-100 chars)
- Use code blocks for JSON/YAML examples
- Use tables for reference information

### 3. Comprehensive Examples

Every section should include examples:

```markdown
## Endpoints

### GET /users/{id}

**Example Request**:
```bash
curl https://api.example.com/v1/users/user-123
```

**Example Response**:
```json
{
  "id": "user-123",
  "name": "John Doe"
}
```
```

### 4. Clear Task Generation Hints

Include comments in your template to guide task generation:

```markdown
<!-- TASK: Generate implementation tasks for each endpoint -->
## Endpoints

[Endpoint definitions]

<!-- TASK: Create test plan covering unit, integration, and e2e -->
## Testing Strategy

[Testing guidance]
```

Note: Comments are removed from final spec but guide Claude in task generation.

### 5. Domain-Specific Information

Include templates for your specific domain:

```markdown
<!-- Payment API specific -->
---
title: "{{ SPEC_TITLE }}"
pci_dss_compliant: true
payment_processors: ["Stripe", "PayPal"]
---

## Payment Processing

[Payment-specific sections]

## Security & Compliance

[PCI-DSS requirements]
```

## Examples

### Example 1: REST API Template

File: `.claude/spec-templates/rest-api.md`

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
version: "1.0.0"
api_type: "REST"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## API Endpoints

[Endpoint structure and examples]

## Request/Response Format

[Standard format]

## Authentication

[Auth mechanism]

## Error Responses

[Error codes and handling]

## Pagination

[Pagination scheme]

## Rate Limiting

[Rate limit details]

## Deployment Checklist

- [ ] Environment configuration
- [ ] Database setup
- [ ] Load balancer configuration
- [ ] Monitoring and logging
- [ ] Documentation deployment
```

### Example 2: Internal Service Template

File: `.claude/spec-templates/internal-service.md`

```markdown
---
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
type: "Internal Service"
owned_by: "{{ USER_NAME }}"
---

# {{ SPEC_TITLE }}

{{ SPEC_DESCRIPTION }}

## Service Info

- **Owner**: {{ USER_NAME }}
- **Spec**: {{ SPEC_ID }}

## Integration Points

[Services that depend on this]

## RPC/API Interface

[Interface definition]

## Data Models

[Schema definitions]

## Deployment

[Deployment procedure]

## Operations

[Runbooks and monitoring]
```

## Troubleshooting

### Template Not Found

**Problem**: Error when specifying custom template

**Solution**:
```bash
# Check available templates
ls .claude/spec-templates/

# Verify template filename matches exactly
/stspec:c --description "..." --template rest-api  # Must match rest-api.md
```

### Template Placeholders Not Filling

**Problem**: Placeholders like `{{ SPEC_ID }}` appear in output

**Solution**:
- Check placeholder spelling (case-sensitive)
- Verify placeholder is in YAML frontmatter
- Supported placeholders: SPEC_ID, SPEC_TITLE, SPEC_DESCRIPTION, CREATED_DATE, USER_NAME

### Taskfiles Not Generated Correctly

**Problem**: Taskfiles don't match template structure

**Solution**:
- Provide detailed description so Claude can understand API complexity
- Use specific API type in template (REST, GraphQL, etc.)
- Include comments in template to guide task generation

## Next Steps

1. **Create your first custom template** based on your API type
2. **Test with a real spec** and refine based on generated tasks
3. **Share templates** with your team in `.claude/spec-templates/`
4. **Iterate** based on experience

---

For the default template structure, see `default-template.md` in this skill's references.
