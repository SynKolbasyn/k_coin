mod main_menu;
mod transfer_menu;
mod states;
mod transfer_history_menu;
mod settings_menu;


use std::io::stdin;

use anyhow::Result;

use crate::key_manager::KeyManager;
use crate::cli::main_menu::MainMenu;
use crate::cli::settings_menu::SettingsMenu;
use crate::cli::states::State;
use crate::cli::transfer_history_menu::TransferHistoryMenu;
use crate::cli::transfer_menu::TransferMenu;


pub struct CLI {
    state: State,
    key_manager: KeyManager,
    main_menu_cli: MainMenu,
    transfer_menu: TransferMenu,
    transfer_history_menu: TransferHistoryMenu,
    settings_menu: SettingsMenu,
}


impl CLI {
    pub fn new() -> Result<CLI> {
        Ok(CLI {
            state: State::MainMenu,
            key_manager: KeyManager::new()?,
            main_menu_cli: MainMenu::new(),
            transfer_menu: TransferMenu::new(),
            transfer_history_menu: TransferHistoryMenu::new(),
            settings_menu: SettingsMenu::new(),
        })
    }

    pub fn show_menu(&mut self) -> Result<()> {
        match self.state {
            State::MainMenu => self.main_menu_cli.show_menu()?,
            State::TransferMenu => self.transfer_menu.show_menu()?,
            State::TransferHistoryMenu => self.transfer_history_menu.show_menu()?,
            State::SettingsMenu => self.settings_menu.show_menu()?,
        }

        Ok(())
    }

    pub fn process_action(&mut self) -> Result<()> {
        let mut action: String = String::new();
        stdin().read_line(&mut action)?;
        action = action.trim().to_string();

        self.state = match self.state {
            State::MainMenu => self.main_menu_cli.process_action(action)?,
            State::TransferMenu => self.transfer_menu.process_action(action)?,
            State::TransferHistoryMenu => self.transfer_history_menu.process_action(action)?,
            State::SettingsMenu => self.settings_menu.process_action(action)?,
        };

        Ok(())
    }
}
