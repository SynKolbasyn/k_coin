use std::io::{stdin, stdout, Write};
use std::cmp::Ordering::Equal;

use anyhow::Result;

use crate::key_manager::KeyManager;


enum State {
    MainMenu,
}


pub struct CLI {
    state: State,
    key_manager: KeyManager,
    main_menu_cli: MainMenuCLI,
}


impl CLI {
    pub fn new() -> Result<CLI> {
        Ok(CLI {
            state: State::MainMenu,
            key_manager: KeyManager::new()?,
            main_menu_cli: MainMenuCLI::new(),
        })
    }

    pub fn show_menu(&self) -> Result<()> {
        match self.state {
            State::MainMenu => self.main_menu_cli.show_menu()?,
        }

        Ok(())
    }

    pub fn process_action(&self) -> Result<()> {
        let mut action: String = String::new();
        stdin().read_line(&mut action)?;

        match self.state {
            State::MainMenu => self.main_menu_cli.process_action(action.trim().to_string()),
        }

        Ok(())
    }
}


struct MainMenuCLI {
    menu: String,
}


impl MainMenuCLI {
    pub fn new() -> MainMenuCLI {
        MainMenuCLI {
            menu: String::new(),
        }
    }

    pub fn show_menu(&self) -> Result<()> {
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&self, action: String) {
        
    }
}
