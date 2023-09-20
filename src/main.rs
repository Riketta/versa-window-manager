mod command_invoker;
mod commands;
mod window;
mod windows_manager;
use crate::{
    command_invoker::*,
    commands::{
        help_command::HelpCommand, list_command::ListCommand, restore_command::RestoreCommand,
        save_command::SaveCommand,
    },
};
use std::env;

const HELP_TEXT: &str = r#"Overview:
This tool allows you to save (as a json file) and restore window states.  
Restoration occurs by searching among existing visible windows for windows equal to one of saved windows with the same title, class and process name.  

Usage:
Run tool with --save [file] flag to dump all visible windows data to dump.json, then edit it as you like (remove unimportant windows and fix position and size of interesting windows for you) and finaly restore window data with --restore [file] flag.  "#;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let default_path = String::from("dump.json");

    let mut invoker: CommandInvoker = CommandInvoker::new();
    invoker.add_command(
        String::from("--help"),
        HelpCommand::new(HELP_TEXT.to_string(), String::from("Print this help.")), // TODO: pass invoker or commands.iter to command constructor?
    );
    invoker.add_command(
        String::from("--list"),
        ListCommand::new(String::from("List data of all visible windows.")),
    );
    invoker.add_command(
        String::from("--save"),
        SaveCommand::new(String::from(&default_path), String::from("Save the data of all visible windows as a json file passed as the first command argument, or otherwise the default path is used.")),
    );
    invoker.add_command(
        String::from("--restore"),
        RestoreCommand::new(String::from(&default_path), String::from("Restore the state of windows (loaded from a file) for existing windows at the moment. The file path is passed as the first argument to the command, or otherwise the default path is used.")),
    );

    if args.len() > 1 {
        let command = String::from(args[1].trim());
        let command_args: CommandArgs;
        command_args = if args.len() > 2 {
            let arg1 = String::from(args[2].trim());
            CommandArgs::String(arg1)
        } else {
            CommandArgs::None
        };
        // println!("{}", command);

        if !invoker.execute(command.to_lowercase(), command_args) {
            print_help(&invoker);
        }
    } else {
        print_help(&invoker);
    }
}

fn print_help(invoker: &CommandInvoker) {
    invoker.execute(
        String::from("--help"),
        CommandArgs::Iter(invoker.get_commands_iter()),
    );
}
