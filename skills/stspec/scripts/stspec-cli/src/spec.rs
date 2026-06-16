use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use chrono::Utc;
use walkdir::WalkDir;
use anyhow::{Result, Context};
use serde_json::json;

use crate::template::TemplateRenderer;
use crate::git::GitManager;

#[derive(Debug, Clone)]
pub struct SpecConfig {
    pub description: String,
    pub title: String,
    pub template_name: Option<String>,
    pub project_root: PathBuf,
    pub dry_run: bool,
}

#[derive(Debug)]
pub struct SpecInfo {
    pub spec_number: u32,
    pub spec_dir_name: String,
    pub spec_file_path: PathBuf,
    pub taskfiles_dir: PathBuf,
    pub git_branch: String,
}

pub struct SpecGenerator {
    config: SpecConfig,
}

impl SpecGenerator {
    pub fn new(config: SpecConfig) -> Result<Self> {
        Ok(SpecGenerator { config })
    }

    pub fn generate(&self) -> Result<SpecInfo> {
        // Find next spec number
        let spec_number = self.find_next_spec_number()?;

        // Sanitize title
        let sanitized_title = sanitize_title(&self.config.title);

        // Create spec directory structure
        let spec_dir_name = format!("SPEC-{:05}-{}", spec_number, sanitized_title);
        let spec_base_dir = self.config.project_root.join("spec");
        let taskfiles_dir = spec_base_dir.join(format!("SPEC-{:05}", spec_number));

        if !self.config.dry_run {
            fs::create_dir_all(&taskfiles_dir)
                .context(format!("Failed to create taskfiles directory at {}", taskfiles_dir.display()))?;
        }

        // Create spec file directly in spec/ directory
        let spec_filename = format!("{}.md", spec_dir_name);
        let spec_file_path = spec_base_dir.join(&spec_filename);

        let renderer = TemplateRenderer::new(&self.config)?;
        let spec_content = renderer.render_spec(
            spec_number,
            &self.config.title,
            &self.config.description,
        )?;

        if !self.config.dry_run {
            fs::write(&spec_file_path, spec_content)
                .context(format!("Failed to write spec file to {}", spec_file_path.display()))?;
        }

        // Create taskfiles (placeholder structure)
        // In a full implementation, Claude AI would generate these
        self.create_default_taskfiles(&taskfiles_dir, spec_number, &sanitized_title)?;

        // Handle git operations
        let git_branch = format!("spec-{:05}-{}", spec_number, sanitized_title);
        let git_manager = GitManager::new(&self.config.project_root)?;

        if !self.config.dry_run {
            git_manager.create_branch(&git_branch)?;
            // Stage the spec file
            git_manager.stage_changes(&spec_file_path)?;
            // Stage the taskfiles directory
            git_manager.stage_changes(&taskfiles_dir)?;
        }

        Ok(SpecInfo {
            spec_number,
            spec_dir_name,
            spec_file_path,
            taskfiles_dir,
            git_branch,
        })
    }

    fn find_next_spec_number(&self) -> Result<u32> {
        let spec_dir = self.config.project_root.join("spec");

        if !spec_dir.exists() {
            return Ok(1);
        }

        let mut max_number = 0u32;
        let re = Regex::new(r"SPEC-(\d+)")?;

        for entry in WalkDir::new(&spec_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_name = entry.file_name().to_string_lossy();
            if let Some(caps) = re.captures(&file_name) {
                if let Ok(num) = caps[1].parse::<u32>() {
                    max_number = max_number.max(num);
                }
            }
        }

        Ok(max_number + 1)
    }

    fn create_default_taskfiles(
        &self,
        taskfiles_dir: &Path,
        spec_number: u32,
        title: &str,
    ) -> Result<()> {
        if self.config.dry_run {
            return Ok(());
        }

        // Create placeholder taskfiles
        let taskfiles = vec![
            (
                "01-features.md",
                format!(
                    "---\ntitle: \"Implementation Features\"\nstory_points: 8\nstatus: \"pending\"\n---\n\n# Implementation Features\n\nBreak down the specification into features.\n\n## Acceptance Criteria\n\n- [ ] Feature 1\n- [ ] Feature 2\n- [ ] Feature 3\n\n## Implementation Notes\n\nAdd notes here."
                ),
            ),
            (
                "02-test-plan.md",
                format!(
                    "---\ntitle: \"Test Plan\"\nstory_points: 5\nstatus: \"pending\"\n---\n\n# Test Plan\n\nTest coverage for the specification.\n\n## Unit Tests\n\n- [ ] Test case 1\n- [ ] Test case 2\n\n## Integration Tests\n\n- [ ] Integration test 1\n- [ ] Integration test 2\n\n## E2E Tests\n\n- [ ] End-to-end test 1\n- [ ] End-to-end test 2"
                ),
            ),
            (
                "03-implementation.md",
                format!(
                    "---\ntitle: \"Implementation Tasks\"\nstory_points: 13\nstatus: \"pending\"\n---\n\n# Implementation Tasks\n\nCore implementation work.\n\n## Setup\n\n- [ ] Initialize project structure\n- [ ] Configure dependencies\n\n## Implementation\n\n- [ ] Implement core functionality\n- [ ] Add error handling\n- [ ] Integrate with dependencies\n\n## Quality\n\n- [ ] Code review\n- [ ] Performance testing"
                ),
            ),
            (
                "04-documentation.md",
                format!(
                    "---\ntitle: \"Documentation\"\nstory_points: 3\nstatus: \"pending\"\n---\n\n# Documentation\n\nDocumentation and guides.\n\n## API Documentation\n\n- [ ] Endpoint documentation\n- [ ] Example requests/responses\n\n## Implementation Guide\n\n- [ ] Setup instructions\n- [ ] Configuration guide\n\n## Runbooks\n\n- [ ] Deployment guide\n- [ ] Troubleshooting guide"
                ),
            ),
        ];

        for (filename, content) in taskfiles {
            let file_path = taskfiles_dir.join(filename);
            fs::write(&file_path, content)
                .context(format!("Failed to write taskfile {}", filename))?;
        }

        Ok(())
    }
}

fn sanitize_title(title: &str) -> String {
    // Replace special characters with hyphens
    let re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    let sanitized = re.replace_all(title, "-");

    // Convert to lowercase and remove leading/trailing hyphens
    sanitized
        .to_lowercase()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_title() {
        assert_eq!(sanitize_title("User Auth API"), "user-auth-api");
        assert_eq!(sanitize_title("REST/GraphQL API"), "rest-graphql-api");
        assert_eq!(sanitize_title("  Spaces  "), "spaces");
    }
}
