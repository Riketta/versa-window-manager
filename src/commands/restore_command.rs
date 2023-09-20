use std::fs::File;
use std::io::prelude::*;

use crate::window::Window;
use crate::{
    command_invoker::{Command, CommandArgs},
    windows_manager::WinManager,
};

pub(crate) struct RestoreCommand {
    default_path: String,
    description: String,
}

impl RestoreCommand {
    pub const fn new(default_path: String, description: String) -> Self {
        RestoreCommand {
            default_path,
            description,
        }
    }
}

impl<'a> Command for RestoreCommand {
    fn execute(&self, arg1: CommandArgs) {
        let path = match arg1 {
            CommandArgs::String(path) => path,
            _ => self.default_path.clone(),
        };

        println!("Reading data from: \"{path}\"");
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        // println!("Restoring data: \"{json}\"");

        let saved_windows: Vec<Window> = serde_json::from_str(&json).unwrap();
        println!("Restoring data for {} window(s).", saved_windows.len());

        let current_windows = WinManager::get_windows_data().unwrap();
        // current_windows.iter_mut().for_each(|w| w.pid = 0);

        // Cases:
        // - one saved window - one existing window: apply saved parameters to existing window.
        // - one saved window - more than one existing windows: apply same saved parameters to all existing window.
        // - more than one saved window (equal or more than existing windows) - more than one existing windows: apply each set of saved parameters to every window.
        // - otherwise: skip window, log error.

        // TODO: not required, replace these two with title regex matching
        // let mut groups_saved_windows: Vec<WindowId> = Vec::new();
        // let mut grouped_saved_window_queues: HashMap<WindowId, VecDeque<&Window>> = HashMap::new();

        // looking for similar windows and form unique window groups based on them (ex: 3 Firefox windows will result in one (firefox.exe, MozillaWindowClass) group).
        // for saved_window in &saved_windows {
        //     //TODO: groups_saved_windows block of code not required, testing stuff
        //     let window_group = saved_window.get_window_group_id();
        //     let is_group_present = groups_saved_windows
        //         .iter()
        //         .any(|group| group == &window_group);

        //     if !is_group_present {
        //         groups_saved_windows.push(window_group.clone());
        //     }

        //     let queue = grouped_saved_window_queues
        //         .entry(window_group)
        //         .or_insert(Default::default());
        //     queue.push_front(saved_window);
        // }

        // iterate existing windows and match each of them with groups of saved windows
        for existing_window in &current_windows {
            let window_id = existing_window.get_window_id();

            // let grouped_saved_window_queue = grouped_saved_window_queues.get_mut(&window_group);
            // let saved_window_queue = match grouped_saved_window_queue {
            //     Some(queue) => queue,
            //     None => {
            //         println!(
            //             "-[{}; Title: \"{}\"; PID: {}] Skipping. No saved data found for window.",
            //             existing_window.get_window_group_id(),
            //             existing_window.title,
            //             existing_window.pid
            //         );
            //         continue;
            //     }
            // };

            // // TODO: clarify error when amount of existing windows of same group greater than saved samples of data for them
            // let saved_window = match saved_window_queue.pop_back() {
            //     Some(val) => val,
            //     None => panic!("called `Option::unwrap()` on a `None` value"),
            // };

            let saved_window = match saved_windows
                .iter()
                .find(|window| window.get_window_id() == window_id)
            {
                Some(val) => val,
                None => {
                    println!(
                        "-[PID: {}; {}] Skipping. No window saved data found.",
                        existing_window.pid, window_id,
                    );
                    continue;
                }
            };

            println!("+[PID: {}; {}] Modifying.", existing_window.pid, window_id);
            WinManager::set_window_position(existing_window.handle, saved_window.window);
        }
    }

    fn get_description(&self) -> &str {
        self.description.as_str()
    }
}
