use serde::{Deserialize, Serialize};
use std::fmt::Display;
use windows::Win32::Foundation::RECT;

/// Window identifier
#[derive(Hash)]
pub struct WindowId<'a> {
    pub process: &'a str,
    pub class: &'a str,
    pub title: &'a str,
}

impl Display for WindowId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process: \"{}\"; Title: \"{}\"; Class: {}",
            self.process, self.title, self.class
        )
    }
}

impl Clone for WindowId<'_> {
    fn clone(&self) -> Self {
        Self {
            process: self.process.clone(),
            title: self.title.clone(),
            class: self.class.clone(),
        }
    }
}

impl PartialEq for WindowId<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self as *const Self) == (other as *const Self) // check if object compared with itself to prevent slow string comparison by fields
            || (self.process == other.process && self.title == other.title && self.class == other.class)
        // TODO: match title with regex
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for WindowId<'_> {}

#[derive(Serialize, Deserialize)]
#[serde(remote = "RECT")]
struct RECTDef {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Window {
    // TODO: store extra window attributes like styles
    pub process_name: String,
    pub pid: u32,
    pub handle: isize,
    pub title: String,
    pub class: String,
    #[serde(with = "RECTDef")]
    pub window: RECT,
}

impl Window {
    pub fn get_window_id(&self) -> WindowId {
        let window_group = WindowId {
            process: &self.process_name,
            title: &self.title,
            class: &self.class,
        };

        window_group
    }

    pub fn get_height(&self) -> i32 {
        self.window.bottom - self.window.top
    }

    pub fn get_width(&self) -> i32 {
        self.window.right - self.window.left
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Process: \"{}\" | PID: {} | Handle: {}] Title: \"{}\"; Class: {}; Position: (X1 = {}, Y1 = {}); (X2 = {}, Y2 = {}); Size: (H{}; W{})", self.process_name, self.pid, self.handle, self.title, self.class, self.window.left, self.window.top, self.window.right, self.window.bottom, self.get_height(), self.get_width())
    }
}
