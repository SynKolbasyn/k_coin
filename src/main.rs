mod cli;
mod key_manager;
mod block;


use crate::block::Block;


use anyhow::Result;

use crate::cli::CLI;


fn main() -> Result<()> {
    let block: Block = Block::new(None, "a", "b", 100.0);
    let ver = block.verify()?;
    println!("{:#?}", block);
    println!("{ver}");
    println!("{}", block.get_bin_hash(ver)?);

    let cli: CLI = CLI::new()?;
    
    loop {
        cli.show_menu()?;
        cli.process_action()?;
    }
}
