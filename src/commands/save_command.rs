use std::fs::File;
use std::io::prelude::*;

use crate::{
    command_invoker::{Command, CommandArgs},
    windows_manager::WinManager,
};

pub(crate) struct SaveCommand {
    default_path: String,
    description: String,
}

impl SaveCommand {
    pub const fn new(default_path: String, description: String) -> Self {
        SaveCommand {
            default_path,
            description,
        }
    }
}

impl<'a> Command for SaveCommand {
    fn execute(&self, arg1: CommandArgs) {
        let path = match arg1 {
            CommandArgs::String(path) => path,
            _ => self.default_path.clone(),
        };

        let mut windows = WinManager::get_windows_data().unwrap();
        windows.iter_mut().for_each(|w| {
            w.pid = 0;
            w.handle = 0
        });

        let json = serde_json::to_string_pretty(&windows).unwrap();

        println!("Saving data as: \"{path}\"");
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_ref()).unwrap();
        println!("Data saved!");
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }
}
