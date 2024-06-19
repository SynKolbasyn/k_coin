use std::{
    // slice::Iter,
    io::{stdout, Write},
};

use anyhow::Result;

use crate::cli::states::State;


#[derive(Clone, Copy)]
enum TransferHistoryMenuAction {
    Update(&'static str),
    Back(&'static str),
}


impl TransferHistoryMenuAction {
    // pub fn iter() -> Iter<'static, MainMenuActions> {
    //     static DIRECTIONS: [MainMenuActions; 3] = [
    //         TransferMoney("Transfer money"),
    //         IncomingPayments("Incoming payments"),
    //         Exit("Exit"),
    //     ];
    //     DIRECTIONS.iter()
    // }

    pub fn iter() -> impl Iterator<Item =TransferHistoryMenuAction> {
        [
            TransferHistoryMenuAction::Update("Update history"),
            TransferHistoryMenuAction::Back("Back to main menu"),
        ].iter().copied()
    }

    pub fn vec() -> Vec<TransferHistoryMenuAction> {
        let mut result: Vec<TransferHistoryMenuAction> = Vec::new();

        for i in Self::iter() {
            result.push(i.clone());
        }

        result
    }

    pub fn get_value(&self) -> String {
        match self {
            TransferHistoryMenuAction::Update(msg) => String::from(*msg),
            TransferHistoryMenuAction::Back(msg) => String::from(*msg),
        }
    }
}


pub struct TransferHistoryMenu {
    menu: String,
}


impl TransferHistoryMenu {
    fn generate_menu() -> String {
        let mut result: String = String::new();

        for (i, e) in TransferHistoryMenuAction::iter().enumerate() {
            result += &format!("[ {} ] -> {}\n", i + 1, e.get_value());
        }

        result + "~$ "
    }

    pub fn new() -> TransferHistoryMenu {
        TransferHistoryMenu {
            menu: Self::generate_menu(),
        }
    }

    pub fn show_menu(&self) -> Result<()> {
        print!("{}", self.menu);
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&mut self, action: String) -> Result<State> {
        let action: TransferHistoryMenuAction = TransferHistoryMenuAction::vec()[action.parse::<usize>()? - 1];

        Ok(match action {
            TransferHistoryMenuAction::Update(_) => self.update_history(),
            TransferHistoryMenuAction::Back(_) => State::MainMenu,
        })
    }
    
    fn update_history(&mut self) -> State {
        State::TransferHistoryMenu
    }
}
