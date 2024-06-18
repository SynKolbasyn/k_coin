mod cli;
mod key_manager;


use anyhow::Result;

use crate::cli::CLI;


fn main() -> Result<()> {
    let cli: CLI = CLI::new()?;
    
    loop {
        cli.show_menu()?;
        cli.process_action()?;
    }
    Ok(())
}
