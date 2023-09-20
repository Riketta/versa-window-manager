use crate::command_invoker::{Command, CommandArgs};

pub(crate) struct HelpCommand {
    help_text: String,
    description: String,
}

impl HelpCommand {
    pub const fn new(help_text: String, description: String) -> Self {
        HelpCommand {
            help_text,
            description,
        }
    }
}

impl<'a> Command for HelpCommand {
    fn execute(&self, arg1: CommandArgs) {
        let iter = match arg1 {
            CommandArgs::Iter(iter) => iter,
            _ => return,
        };

        // println!("Help:");
        println!("{}", self.help_text);
        println!();
        println!("Commands:");
        for command in iter {
            println!("{} - {}", command.0, command.1.get_description());
        }
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }
}
