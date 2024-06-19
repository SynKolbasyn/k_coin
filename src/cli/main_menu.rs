use std::{
    // slice::Iter,
    io::{stdout, Write},
    process::exit,
};

use anyhow::Result;

use crate::cli::states::State;


#[derive(Clone, Copy)]
enum MainMenuAction {
    TransferMoney(&'static str),
    TransferHistory(&'static str),
    Exit(&'static str),
}


impl MainMenuAction {
    // pub fn iter() -> Iter<'static, MainMenuActions> {
    //     static DIRECTIONS: [MainMenuActions; 3] = [
    //         TransferMoney("Transfer money"),
    //         IncomingPayments("Incoming payments"),
    //         Exit("Exit"),
    //     ];
    //     DIRECTIONS.iter()
    // }

    pub fn iter() -> impl Iterator<Item =MainMenuAction> {
        [
            MainMenuAction::TransferMoney("Transfer money"),
            MainMenuAction::TransferHistory("View transfer history"),
            MainMenuAction::Exit("Exit"),
        ].iter().copied()
    }

    pub fn vec() -> Vec<MainMenuAction> {
        let mut result: Vec<MainMenuAction> = Vec::new();

        for i in Self::iter() {
            result.push(i.clone());
        }

        result
    }

    pub fn get_value(&self) -> String {
        match self {
            MainMenuAction::TransferMoney(msg) => String::from(*msg),
            MainMenuAction::TransferHistory(msg) => String::from(*msg),
            MainMenuAction::Exit(msg) => String::from(*msg),
        }
    }
}


pub struct MainMenuCLI {
    menu: String,
}


impl MainMenuCLI {
    fn generate_menu() -> String {
        let mut result: String = String::new();

        for (i, e) in MainMenuAction::iter().enumerate() {
            result += &format!("[ {} ] -> {}\n", i + 1, e.get_value());
        }

        result + "~$ "
    }

    pub fn new() -> MainMenuCLI {
        MainMenuCLI {
            menu: Self::generate_menu(),
        }
    }

    pub fn show_menu(&self) -> Result<()> {
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&self, action: String) -> Result<State> {
        let action: MainMenuAction = MainMenuAction::vec()[action.parse::<usize>()? - 1];

        Ok(match action {
            MainMenuAction::TransferMoney(_) => State::TransferMenu,
            MainMenuAction::TransferHistory(_) => State::TransferHistoryMenu,
            MainMenuAction::Exit(_) => exit(0),
        })
    }
}
