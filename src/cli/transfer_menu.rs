use std::{
    // slice::Iter,
    io::{stdout, Write},
};
use std::cmp::Ordering::Equal;

use anyhow::Result;

use crate::cli::states::State;


const RECIPIENT_ADDRESS_MESSAGE: &str = "Enter the recipient's address: ";
const AMOUNT_MESSAGE: &str = "Enter amount: ";
const CONFIRMATION_MESSAGE: &str = "Confirm transfer [Yes/no]?: ";


enum TransferState {
    EnteringAddressee(&'static str),
    EnteringAmount(&'static str),
    ConfirmingTransfer(&'static str),
}


impl TransferState {
    fn get_val(&self) -> String {
        match self {
            TransferState::EnteringAddressee(msg) => String::from(*msg),
            TransferState::EnteringAmount(msg) => String::from(*msg),
            TransferState::ConfirmingTransfer(msg) => String::from(*msg),
        }
    }
}


pub struct TransferMenu {
    state: TransferState,
    addressee: String,
    amount: f64,
}


impl TransferMenu {
    pub fn new() -> TransferMenu {
        TransferMenu {
            state: TransferState::EnteringAddressee(RECIPIENT_ADDRESS_MESSAGE),
            addressee: String::new(),
            amount: 0.0,
        }
    }

    pub fn show_menu(&self) -> Result<()> {
        print!("{}", self.state.get_val());
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&mut self, action: String) -> Result<State> {
        Ok(match self.state {
            TransferState::EnteringAddressee(_) => self.set_addressee(action),
            TransferState::EnteringAmount(_) => self.set_amount(action.parse::<f64>()?),
            TransferState::ConfirmingTransfer(_) => self.confirm_transfer(action),
        })
    }
    
    fn set_addressee(&mut self, addressee: String) -> State {
        self.addressee = addressee;
        self.state = TransferState::EnteringAmount(AMOUNT_MESSAGE);
        State::TransferMenu
    }
    
    fn set_amount(&mut self, amount: f64) -> State {
        self.amount = amount;
        self.state = TransferState::ConfirmingTransfer(CONFIRMATION_MESSAGE);
        State::TransferMenu
    }
    
    fn confirm_transfer(&mut self, confirmation: String) -> State {
        let confirmation_strings: [String; 4] = [
            String::from("Yes"),
            String::from("yes"),
            String::from("Y"),
            String::from("y"),
        ];
        
        self.state = TransferState::EnteringAddressee(RECIPIENT_ADDRESS_MESSAGE);
        
        for i in confirmation_strings {
            if confirmation.cmp(&i) == Equal {
                println!("CONFIRMATION DONE");
                return State::MainMenu;
            }
        }
        
        println!("CONFIRMATION DENIED");
        State::MainMenu
    }
}
