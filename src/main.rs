use clap::{Parser, Subcommand};
use std::{
    fs::{self},
    io::Write,
    path::Path,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add {
        service: String,
        username: String,
        password: String,
    },
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.cmd {
        Commands::List => display_passwords(),
        Commands::Add {
            service,
            username,
            password,
        } => add_new_password(service, username, password)?,
    }
    Ok(())
}

fn display_passwords() {
    let path = Path::new("./passwords.txt");
    let contents = fs::read_to_string(path).expect("Could not read the file");
    println!("{}", contents);
}

fn add_new_password(service: String, username: String, password: String) -> std::io::Result<()> {
    let path = Path::new("./passwords.txt")
        .canonicalize()
        .expect("Could not find the file");
    let password_infos = format!("{}|{}|{}\n", service, username, password);

    let mut file = fs::OpenOptions::new().append(true).open(path)?;

    file.write_all(password_infos.as_bytes())?;

    Ok(())
}
