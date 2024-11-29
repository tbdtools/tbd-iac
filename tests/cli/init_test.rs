use anyhow::Result;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[tokio::test]
async fn test_init_creates_project_structure() -> Result<()> {
    // Create a temporary directory for our test
    let temp = assert_fs::TempDir::new()?;

    // Run init command
    tbd_iac::cli::handle_init("test-project", &Some(temp.path().to_path_buf())).await?;

    // Verify directory structure
    temp.child("stacks").assert(predicate::path::is_dir());
    temp.child("modules").assert(predicate::path::is_dir());
    temp.child("providers").assert(predicate::path::is_dir());
    temp.child("tests").assert(predicate::path::is_dir());

    // Verify file contents
    temp.child("pyproject.toml")
        .assert(predicate::path::exists());
    temp.child("README.md").assert(predicate::path::exists());
    temp.child(".gitignore").assert(predicate::path::exists());
    temp.child("stacks/main.py")
        .assert(predicate::path::exists());

    // Check pyproject.toml content
    let pyproject = temp.child("pyproject.toml");
    pyproject.assert(predicate::str::contains("name = \"test-project\""));
    pyproject.assert(predicate::str::contains("tbdtools = \"^0.1.0\""));

    Ok(())
}

#[tokio::test]
async fn test_init_handles_existing_directory() -> Result<()> {
    let temp = assert_fs::TempDir::new()?;

    // Create an existing file
    temp.child("existing.txt").write_str("test")?;

    // Run init command - should not fail with existing directory
    tbd_iac::cli::handle_init("test-project", &Some(temp.path().to_path_buf())).await?;

    // Verify both new and existing files
    temp.child("existing.txt").assert(predicate::path::exists());
    temp.child("pyproject.toml")
        .assert(predicate::path::exists());

    Ok(())
}
