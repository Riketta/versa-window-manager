use crate::{
    command_invoker::{Command, CommandArgs},
    windows_manager::WinManager,
};

pub(crate) struct ListCommand {
    description: String,
}

impl ListCommand {
    pub const fn new(description: String) -> Self {
        ListCommand { description }
    }
}

impl<'a> Command for ListCommand {
    fn execute(&self, _arg1: CommandArgs) {
        let windows = match WinManager::get_windows_data() {
            Ok(result) => result,
            Err(_) => todo!(), // TODO: process error
        };

        for window in windows {
            println!("{}", window);
        }
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }
}
