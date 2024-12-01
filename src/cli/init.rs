use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub async fn handle_init(name: &str, dir: &Option<PathBuf>) -> Result<()> {
    let project_dir = dir.clone().unwrap_or_else(|| PathBuf::from(name));
    create_project_structure(&project_dir, name)?;
    Ok(())
}

fn create_project_structure(project_dir: &Path, name: &str) -> Result<()> {
    // Create main project directory
    fs::create_dir_all(project_dir)
        .with_context(|| format!("Failed to create project directory at {:?}", project_dir))?;

    // Create project subdirectories
    let dirs = ["stacks", "modules", "providers", "tests"];
    for dir in dirs.iter() {
        fs::create_dir_all(project_dir.join(dir))
            .with_context(|| format!("Failed to create {} directory", dir))?;
    }

    // Create pyproject.toml
    let pyproject_content = format!(
        r#"[tool.poetry]
name = "{}"
version = "0.1.0"
description = "Infrastructure as Code project using TBD Tools"

[tool.poetry.dependencies]
python = "^3.9"
tbdtools = "^0.1.0"

[tool.poetry.dev-dependencies]
pytest = "^7.0"
black = "^23.0"
mypy = "^1.0"
"#,
        name
    );

    fs::write(project_dir.join("pyproject.toml"), pyproject_content)
        .context("Failed to create pyproject.toml")?;

    // Create example stack
    let example_stack = r#"from tbdtools import (
    Stack,
    aws,
    Resource,
    Output,
)

class MainStack(Stack):
    def __init__(self, name: str):
        super().__init__(name)
        
        # Example VPC
        vpc = aws.ec2.Vpc(
            self,
            "MainVpc",
            cidr="10.0.0.0/16",
            max_azs=2,
        )
        
        # Example outputs
        self.output(
            "vpc_id",
            value=vpc.vpc_id,
            description="ID of the VPC"
        )

# Initialize stack
stack = MainStack("main")
"#
    .to_string();

    fs::write(project_dir.join("stacks").join("main.py"), example_stack)
        .context("Failed to create example stack")?;

    // Create README.md
    let readme_content = format!(
        r#"# {}

Infrastructure as Code project using TBD Tools.

## Project Structure

```
.
├── stacks/          # Stack definitions
├── modules/         # Reusable infrastructure modules
├── providers/       # Custom provider configurations
└── tests/          # Infrastructure tests
```

## Getting Started

1. Install dependencies:
   ```bash
   poetry install
   ```

2. Initialize providers:
   ```bash
   tbd provider install aws
   ```

3. Deploy infrastructure:
   ```bash
   tbd plan -s main
   tbd apply -s main
   ```
"#,
        name
    );

    fs::write(project_dir.join("README.md"), readme_content)
        .context("Failed to create README.md")?;

    // Create .gitignore
    let gitignore_content = r#".env
*.pyc
__pycache__/
.pytest_cache/
.coverage
*.tfstate
*.tfstate.*
.terraform/
.tbd/
"#;

    fs::write(project_dir.join(".gitignore"), gitignore_content)
        .context("Failed to create .gitignore")?;

    Ok(())
}
