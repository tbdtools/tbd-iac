use tbdiac::cmd::new::new_project;
use tbdiac::startup::run;

fn main() {
    let matches = run().get_matches();
    match matches.subcommand() {
        Some(("new", new_matches)) => {
            let name = new_matches
                .get_one::<String>("name")
                .expect("name is required");
            let path = new_matches
                .get_one::<String>("path")
                .expect("path is required");
            new_project(name, path).unwrap();
        }
        _ => {
            println!("No subcommand provided");
        }
    }
}
