/// Create a new project
pub fn new_project(name: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // the new subcommand implementation will go here
    //let cwd = std::env::current_dir()?;
    println!("Creating a new project: {} in {}", name, path);
    Ok(())
}
