use std::collections::{hash_map::Iter, HashMap};

pub trait TargetObject {}

pub enum CommandArgs<'a> {
    None,
    // Path(Box<Path>),
    String(String),
    Iter(Iter<'a, String, Box<dyn Command>>),
}
pub trait Command {
    fn execute(&self, arg1: CommandArgs);
    fn get_description(&self) -> &str;
}

pub(crate) struct CommandInvoker {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandInvoker {
    pub fn new() -> Self {
        CommandInvoker {
            commands: HashMap::new(),
        }
    }

    pub fn add_command(&mut self, command_name: String, command: impl Command + 'static) {
        self.commands
            .entry(command_name)
            .or_insert(Box::new(command));
    }

    // TODO: use args: Vec<CommandArgs> instead of single arg1
    pub fn execute(&self, command_name: String, arg1: CommandArgs) -> bool {
        if !self.commands.contains_key(&command_name) {
            return false;
        }

        let command: Option<&Box<dyn Command>> = self.commands.get(&command_name);
        command.unwrap().execute(arg1); // TODO: handle unwrap
        true
    }

    pub fn get_commands_iter(&self) -> Iter<'_, String, Box<dyn Command>> {
        self.commands.iter()
    }
}

impl Default for CommandInvoker {
    fn default() -> Self {
        Self::new()
    }
}
