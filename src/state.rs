use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Luma {
    /// First element is the name second is the links
    pub tabs: Vec<(String, Vec<Link>)>,
}

// #[derive(Default)]
// pub struct State {
//     pub selected_tab: usize,
//     pub selected_index: usize,
//     pub unsaved_changes: bool,
// }

#[derive(Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub name: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<String>,
}

// impl Link {
//     pub fn new(name: impl Into<String>, link: impl Into<String>) -> Link {
//         Link {
//             name: name.into(),
//             link: link.into(),
//             file: None,
//             desc: None,
//             artist: None,
//             color: None,
//         }
//     }
// }

#[derive(Debug)]
pub struct OpenCommand {
    pub name: &'static str,
    pub args: &'static [&'static str],
}

#[derive(Debug)]
pub struct OpenCommandError;
impl fmt::Display for OpenCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("failed to spawn command")
    }
}
impl Context for OpenCommandError {}

impl OpenCommand {
    pub const fn new(name: &'static str, args: &'static [&'static str]) -> Self {
        Self { name, args }
    }

    pub fn run(&self, name: &str) -> Result<std::process::Child, OpenCommandError> {
        // it isn't really out concern right now how the process went

        let mut child = Command::new(self.name);

        child
            .args(self.args.iter())
            .arg(name)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let mut c = child.spawn().change_context(OpenCommandError)?;
        // drop stdin so process doesn't wait for input
        let _ = c.stdout.take();
        Ok(c)
    }

    pub fn spawn(&self, name: &str) -> Result<std::process::Child, OpenCommandError> {
        // it isn't really out concern right now how the process went

        let mut child = Command::new(self.name);

        child.args(self.args.iter()).arg(name);

        let mut c = child.spawn().change_context(OpenCommandError)?;
        // drop stdin so process doesn't wait for input
        let _ = c.stdout.take();
        Ok(c)
    }
}
