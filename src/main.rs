mod commands;
mod crypto;
mod storage;
mod ui;

use clap::Parser;
use commands::{Cli, Commands};
use std::io;

fn main() -> io::Result<()> {
    let args = Cli::parse();

    match args.cmd {
        Commands::List => storage::display_passwords()?,
        Commands::Add {
            service,
            username,
            clipboard,
            generate,
            write,
        } => {
            if generate {
                let password = crypto::generate_password();
                println!("{}", password);
                let _ = storage::add_new_password(&service, &username, &password);
            }

            if clipboard {
                let password = ui::get_clipboard_password();
                println!("{}", password);
                if ui::should_save_password(&password) {
                    let _ = storage::add_new_password(&service, &username, &password);
                }
            }

            if write {
                let password = ui::get_user_input().unwrap();
                if ui::should_save_password(&password) {
                    let _ = storage::add_new_password(&service, &username, &password);
                }
            }
        }
    }
    Ok(())
}
