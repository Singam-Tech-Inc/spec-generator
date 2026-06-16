use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, Context};

pub struct GitManager {
    repo_path: PathBuf,
}

impl GitManager {
    pub fn new(repo_path: &Path) -> Result<Self> {
        // Verify this is a git repository
        if !repo_path.join(".git").exists() {
            return Err(anyhow::anyhow!(
                "Not a git repository at {}",
                repo_path.display()
            ));
        }

        Ok(GitManager {
            repo_path: repo_path.to_path_buf(),
        })
    }

    pub fn create_branch(&self, branch_name: &str) -> Result<()> {
        // Check if branch already exists
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .arg("branch")
            .arg("--list")
            .arg(branch_name)
            .output()
            .context("Failed to check git branch")?;

        let branch_exists = !String::from_utf8_lossy(&output.stdout).is_empty();

        if !branch_exists {
            // Create new branch
            let result = Command::new("git")
                .arg("-C")
                .arg(&self.repo_path)
                .arg("checkout")
                .arg("-b")
                .arg(branch_name)
                .output()
                .context(format!("Failed to create git branch: {}", branch_name))?;

            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                return Err(anyhow::anyhow!("Git error: {}", stderr));
            }
        } else {
            // Switch to existing branch
            let result = Command::new("git")
                .arg("-C")
                .arg(&self.repo_path)
                .arg("checkout")
                .arg(branch_name)
                .output()
                .context(format!("Failed to checkout branch: {}", branch_name))?;

            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                return Err(anyhow::anyhow!("Git error: {}", stderr));
            }
        }

        Ok(())
    }

    pub fn stage_changes(&self, paths: &Path) -> Result<()> {
        let relative_path = paths
            .strip_prefix(&self.repo_path)
            .unwrap_or(paths)
            .to_string_lossy();

        let result = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .arg("add")
            .arg(relative_path.as_ref())
            .output()
            .context("Failed to stage git changes")?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(anyhow::anyhow!("Git error: {}", stderr));
        }

        Ok(())
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        let result = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .context("Failed to commit changes")?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(anyhow::anyhow!("Git error: {}", stderr));
        }

        Ok(())
    }

    pub fn get_current_branch(&self) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .context("Failed to get current branch")?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_manager_invalid_repo() {
        let result = GitManager::new(Path::new("/tmp/nonexistent"));
        assert!(result.is_err());
    }
}
