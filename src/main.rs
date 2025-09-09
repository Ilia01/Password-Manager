mod commands;
mod crypto;
mod storage;
mod ui;

use clap::Parser;
use commands::{Cli, Commands};
use core::panic;
use std::io;

use crate::{
    crypto::{derive_key, generate_password, load_or_generate_salt},
    storage::add_new_password,
    ui::{
        get_clipboard_password, get_master_password_user_input, get_user_input,
        should_save_password,
    },
};

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let master_password = get_master_password_user_input()?;

    let salt = load_or_generate_salt();

    let key = derive_key(&master_password, &salt);

    match args.cmd {
        Commands::List => storage::display_passwords(&key)?,

        Commands::Add {
            service,
            username,
            clipboard,
            generate,
            write,
        } => {
            let password = if generate {
                generate_password()
            } else if clipboard {
                get_clipboard_password()
            } else if write {
                get_user_input()?
            } else {
                panic!("No Password Input method selected");
            };

            if should_save_password(&password) {
                add_new_password(&service, &username, &password, &key)?;
            }
        }
    }
    Ok(())
}
