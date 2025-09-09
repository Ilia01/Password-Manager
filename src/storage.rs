use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

pub fn display_passwords() -> std::io::Result<()> {
    let path = Path::new("./passwords.txt");
    let contents = fs::read_to_string(path).expect("Could not read the passwords file");
    println!("{}", contents);
    Ok(())
}

pub fn add_new_password(service: &str, username: &str, password: &str) -> std::io::Result<()> {
    let path = Path::new("./passwords.txt");
    let password_infos = format!("{}|{}|{}\n", service, username, password);
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(password_infos.as_bytes())?;
    Ok(())
}
