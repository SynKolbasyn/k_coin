use std::{
    // slice::Iter,
    io::{stdout, Write},
    collections::HashMap,
};

use anyhow::Result;

use crate::cli::states::State;


const ENABLE_ONE_CPU_MINING_MSG: &str = "Enable mining on one cpu core";
const ENABLE_ALL_CPU_MINING_MSG: &str = "Enable mining on all cpu cores";
const BACK_MSG: &str = "Back";


#[derive(Clone, Copy)]
enum SettingsMenuAction {
    EnableOneCpuMining(&'static str),
    EnableAllCpusMining(&'static str),
    Back(&'static str),
}


impl SettingsMenuAction {
    // pub fn iter() -> Iter<'static, MainMenuActions> {
    //     static DIRECTIONS: [MainMenuActions; 3] = [
    //         TransferMoney("Transfer money"),
    //         IncomingPayments("Incoming payments"),
    //         Exit("Exit"),
    //     ];
    //     DIRECTIONS.iter()
    // }

    pub fn iter() -> impl Iterator<Item =SettingsMenuAction> {
        [
            SettingsMenuAction::EnableOneCpuMining(ENABLE_ONE_CPU_MINING_MSG),
            SettingsMenuAction::EnableAllCpusMining(ENABLE_ALL_CPU_MINING_MSG),
            SettingsMenuAction::Back(BACK_MSG),
        ].iter().copied()
    }

    pub fn vec() -> Vec<SettingsMenuAction> {
        let mut result: Vec<SettingsMenuAction> = Vec::new();

        for i in Self::iter() {
            result.push(i.clone());
        }

        result
    }

    pub fn get_value(&self) -> String {
        match self {
            SettingsMenuAction::EnableOneCpuMining(msg) => String::from(*msg),
            SettingsMenuAction::EnableAllCpusMining(msg) => String::from(*msg),
            SettingsMenuAction::Back(msg) => String::from(*msg),
        }
    }
}


pub struct SettingsMenu {
    settings: HashMap<String, bool>,
}


impl SettingsMenu {
    pub fn new() -> SettingsMenu {
        SettingsMenu {
            settings: HashMap::new(),
        }
    }

    pub fn show_menu(&mut self) -> Result<()> {
        print!("{}", self.generate_menu());
        stdout().flush()?;
        Ok(())
    }

    pub fn process_action(&mut self, action: String) -> Result<State> {
        let action: SettingsMenuAction = SettingsMenuAction::vec()[action.parse::<usize>()? - 1];

        Ok(match action {
            SettingsMenuAction::EnableOneCpuMining(_) => self.change_one_cpu_mining(),
            SettingsMenuAction::EnableAllCpusMining(_) => self.change_all_cpu_mining(),
            SettingsMenuAction::Back(_) => State::MainMenu,
        })
    }

    fn generate_menu(&mut self) -> String {
        let mut result: String = String::new();

        for (i, e) in SettingsMenuAction::iter().enumerate() {
            let desc: String = e.get_value();
            result += &format!("[ {} ] [{}]-> {}\n", i + 1, if *self.settings.entry(desc.clone()).or_insert(false) { "*" } else { " " }, desc);
        }

        result + "~$ "
    }
    
    fn change_one_cpu_mining(&mut self) -> State {
        self.settings.insert(ENABLE_ALL_CPU_MINING_MSG.to_string(), false);
        self.settings.insert(ENABLE_ONE_CPU_MINING_MSG.to_string(), self.settings[&ENABLE_ONE_CPU_MINING_MSG.to_string()] ^ true);
        State::SettingsMenu
    }

    fn change_all_cpu_mining(&mut self) -> State {
        self.settings.insert(ENABLE_ONE_CPU_MINING_MSG.to_string(), false);
        self.settings.insert(ENABLE_ALL_CPU_MINING_MSG.to_string(), self.settings[&ENABLE_ALL_CPU_MINING_MSG.to_string()] ^ true);
        State::SettingsMenu
    }
}
