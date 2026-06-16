use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use chrono::Utc;
use walkdir::WalkDir;
use anyhow::{Result, Context};
use std::process::Command;

mod spec;
mod git;
mod template;

use spec::{SpecGenerator, SpecConfig};

#[derive(Parser, Debug)]
#[command(name = "stspec")]
#[command(about = "Generate API specifications with auto-numbering and git integration", long_about = None)]
struct Args {
    /// Specification description (required)
    #[arg(short, long)]
    description: String,

    /// Specification title (optional, auto-generated from description if not provided)
    #[arg(short, long)]
    title: Option<String>,

    /// Custom template name (looks in .claude/spec-templates/)
    #[arg(short, long)]
    template: Option<String>,

    /// Project root path (overrides .claude/settings.json and auto-detection)
    #[arg(short, long)]
    project_root: Option<PathBuf>,

    /// Don't create git branch or commit (dry-run mode)
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate inputs
    if args.description.is_empty() {
        eprintln!("Error: Description is required");
        return Ok(());
    }

    // Generate title if not provided (first 50 chars of description)
    let title = args.title.unwrap_or_else(|| {
        args.description
            .chars()
            .take(50)
            .collect::<String>()
            .trim()
            .to_string()
    });

    // Find project root
    let project_root = if let Some(pr) = args.project_root {
        pr
    } else {
        find_project_root()?
    };

    // Create spec generator
    let config = SpecConfig {
        description: args.description.clone(),
        title: title.clone(),
        template_name: args.template.clone(),
        project_root: project_root.clone(),
        dry_run: args.dry_run,
    };

    let generator = SpecGenerator::new(config)?;

    // Generate spec
    println!("🔧 Generating specification...");
    let spec_info = generator.generate()?;

    // Output results
    println!("\n✅ Specification created successfully!\n");
    println!("📁 Directory: spec/{}/", spec_info.spec_dir_name);
    println!("📄 Spec File: {}", spec_info.spec_file_path.display());
    println!("📋 Taskfiles: {}", spec_info.taskfiles_dir.display());
    println!("🌿 Git Branch: {}", spec_info.git_branch);

    if !args.dry_run {
        println!("\n📌 Files created and staged.");
        println!("✔️  Ready for commit to branch: {}", spec_info.git_branch);
    }

    println!("\n💡 Next steps:");
    println!("  1. Review generated taskfiles in: {}", spec_info.taskfiles_dir.display());
    println!("  2. Edit taskfiles or provide feedback for refinement");
    println!("  3. Run 'git add -A && git commit' when ready");

    Ok(())
}

fn find_project_root() -> Result<PathBuf> {
    // Try to read project_root from .claude/settings.json
    if let Ok(path) = read_project_root_from_settings() {
        return Ok(path);
    }

    // Search upward for .git directory
    let mut current = std::env::current_dir()?;

    loop {
        if current.join(".git").exists() {
            return Ok(current);
        }

        if !current.pop() {
            return Err(anyhow::anyhow!(
                "Git repository not found. Please:\n  \
                 1. Initialize git: git init\n  \
                 2. Or set project_root in .claude/settings.json\n  \
                 3. Or use --project-root argument"
            ));
        }
    }
}

fn read_project_root_from_settings() -> Result<PathBuf> {
    let settings_path = PathBuf::from(".claude/settings.json");
    let content = fs::read_to_string(&settings_path)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    if let Some(root) = json.get("project_root").and_then(|v| v.as_str()) {
        Ok(PathBuf::from(root))
    } else {
        Err(anyhow::anyhow!("project_root not found in .claude/settings.json"))
    }
}
