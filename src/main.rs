mod cli;
mod key_manager;
mod block;
mod blockchain;


use anyhow::Result;

use crate::cli::CLI;


fn main() -> Result<()> {
    let cli: CLI = CLI::new()?;
    
    loop {
        cli.show_menu()?;
        cli.process_action()?;
    }
}
