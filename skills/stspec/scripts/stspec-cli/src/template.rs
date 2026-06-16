use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use chrono::Utc;
use serde_json::json;

use crate::spec::SpecConfig;

pub struct TemplateRenderer {
    template_content: String,
}

impl TemplateRenderer {
    pub fn new(config: &SpecConfig) -> Result<Self> {
        let template_content = if let Some(template_name) = &config.template_name {
            // Try to load custom template
            Self::load_custom_template(&config.project_root, template_name)?
        } else {
            // Use default template
            Self::get_default_template()
        };

        Ok(TemplateRenderer { template_content })
    }

    pub fn render_spec(
        &self,
        spec_number: u32,
        title: &str,
        description: &str,
    ) -> Result<String> {
        let mut content = self.template_content.clone();
        let created_date = Utc::now().to_rfc3339();

        // Replace placeholders
        content = content.replace("{{ SPEC_ID }}", &format!("SPEC-{:05}", spec_number));
        content = content.replace("{{ SPEC_TITLE }}", title);
        content = content.replace("{{ SPEC_DESCRIPTION }}", description);
        content = content.replace("{{ CREATED_DATE }}", &created_date);

        Ok(content)
    }

    fn load_custom_template(project_root: &Path, template_name: &str) -> Result<String> {
        let template_path = project_root
            .join(".claude")
            .join("spec-templates")
            .join(format!("{}.md", template_name));

        if !template_path.exists() {
            // Try without .md extension
            let template_path_alt = project_root
                .join(".claude")
                .join("spec-templates")
                .join(template_name);

            if !template_path_alt.exists() {
                return Err(anyhow::anyhow!(
                    "Template '{}' not found in .claude/spec-templates/\n\
                     Available templates: {:?}",
                    template_name,
                    Self::list_available_templates(project_root)?
                ));
            }

            return fs::read_to_string(&template_path_alt)
                .context(format!("Failed to read template from {}", template_path_alt.display()));
        }

        fs::read_to_string(&template_path)
            .context(format!("Failed to read template from {}", template_path.display()))
    }

    fn list_available_templates(project_root: &Path) -> Result<Vec<String>> {
        let templates_dir = project_root.join(".claude").join("spec-templates");

        if !templates_dir.exists() {
            return Ok(vec![]);
        }

        let mut templates = vec![];
        for entry in fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem() {
                    templates.push(name.to_string_lossy().to_string());
                }
            }
        }

        Ok(templates)
    }

    fn get_default_template() -> String {
        // Return a minimal default template
        // In production, this would be the full template from references/default-template.md
        r#"---
layout: default
title: "{{ SPEC_TITLE }}"
spec_id: "{{ SPEC_ID }}"
version: "1.0.0"
description: "{{ SPEC_DESCRIPTION }}"
created_date: "{{ CREATED_DATE }}"
status: draft
---

# {{ SPEC_TITLE }}

## Overview

{{ SPEC_DESCRIPTION }}

### Quick Facts

- **Base URL**: `https://api.example.com/v1`
- **Authentication**: OAuth2 / JWT / API Key
- **Response Format**: JSON

## Endpoints

### Base Structure

All endpoints follow this pattern:
```
[METHOD] /v1/[resource]/[action]
```

## Request/Response Format

### Request Structure

All requests must include:
- **Authentication**: Bearer token in Authorization header
- **Content-Type**: application/json (for POST/PUT/PATCH)

### Response Structure

All responses follow this envelope:

```json
{
  "status": "success|error",
  "code": "SUCCESS|ERROR_CODE",
  "data": { /* Response payload */ },
  "meta": {
    "request_id": "unique-request-id",
    "timestamp": "{{ CREATED_DATE }}"
  }
}
```

## Error Codes and Status Codes

### HTTP Status Codes

| Code | Name | Description |
|------|------|-------------|
| 200 | OK | Successful request |
| 201 | Created | Resource created successfully |
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Authentication required or failed |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

## Authentication & Authorization

### OAuth2

**Flow**: Authorization Code with PKCE

**Token Response**:
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

## Rate Limiting

### Rate Limit Headers

All responses include rate limit information:

```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642258200
```

---

**Document Status**: Draft
**Last Updated**: {{ CREATED_DATE }}
"#.to_string()
    }
}
