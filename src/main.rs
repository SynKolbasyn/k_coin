mod cli;
mod key_manager;
mod block;
mod blockchain;


use std::process::exit;

use anyhow::Result;

use crate::cli::CLI;


fn main() {
    loop {
        match main_cli_loop() {
            Ok(_) => exit(0),
            Err(e) => eprintln!("CRITICAL ERROR: {e}"),
        }
    }
}


fn main_cli_loop() -> Result<()> {
    let mut cli: CLI = CLI::new()?;
    
    loop {
        cli.show_menu()?;
        cli.process_action()?;
    }
}
