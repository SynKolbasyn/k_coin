use std::{
    // slice::Iter,
    io::{stdin, stdout, Write},
    process::exit,
};

use anyhow::Result;

use crate::key_manager::KeyManager;
use crate::blockchain::Blockchain;


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
            State::MainMenu => self.main_menu_cli.process_action(action.trim().to_string())?,
        }

        Ok(())
    }
}


/// Main menu cli


#[derive(Debug, Copy, Clone)]
enum MainMenuActions {
    TransferMoney(&'static str),
    IncomingMessages(&'static str),
    Exit(&'static str),
}


impl MainMenuActions {
    // pub fn iter() -> Iter<'static, MainMenuActions> {
    //     static DIRECTIONS: [MainMenuActions; 3] = [
    //         TransferMoney("Transfer money"),
    //         IncomingPayments("Incoming payments"),
    //         Exit("Exit"),
    //     ];
    //     DIRECTIONS.iter()
    // }

    pub fn iter() -> impl Iterator<Item = MainMenuActions> {
        [
            MainMenuActions::TransferMoney("Transfer money"),
            MainMenuActions::IncomingMessages("Incoming messages"),
            MainMenuActions::Exit("Exit"),
        ].iter().copied()
    }
    
    pub fn vec() -> Vec<MainMenuActions> {
        let mut result: Vec<MainMenuActions> = Vec::new();
        
        for i in Self::iter() {
            result.push(i.clone());
        }
        
        result
    }
    
    pub fn get_value(&self) -> String {
        match self {
            MainMenuActions::TransferMoney(msg) => String::from(*msg),
            MainMenuActions::IncomingMessages(msg) => String::from(*msg),
            MainMenuActions::Exit(msg) => String::from(*msg),
        }
    }
}


struct MainMenuCLI {
    menu: String,
}


impl MainMenuCLI {
    fn generate_menu() -> String {
        let mut result: String = String::new();

        for (i, e) in MainMenuActions::iter().enumerate() {
            result += &format!("[ {} ] -> {}\n", i + 1, e.get_value());
        }

        result + "~$ "
    }

    pub fn new() -> MainMenuCLI {
        MainMenuCLI {
            menu: Self::generate_menu()
        }
    }

    pub fn show_menu(&self) -> Result<()> {
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&self, action: String) -> Result<()> {
        let action: MainMenuActions = MainMenuActions::vec()[action.parse::<usize>()? - 1];
        
        match action {
            MainMenuActions::TransferMoney(_) => (),
            MainMenuActions::IncomingMessages(_) => (),
            MainMenuActions::Exit(_) => exit(0),
        }
        
        Ok(())
    }
}
