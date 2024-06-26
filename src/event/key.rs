use std::fmt;

use crossterm::event::KeyEvent;

/// Represents an key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    /// Tabulation key
    Tab,
    ShiftTab,
    /// Backspace key
    Backspace,
    /// Escape key
    Esc,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Insert key
    Ins,
    /// Delete key
    Delete,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,

    /// F key
    F(u8),
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "{}", c),
            Key::Left | Key::Right | Key::Up | Key::Down => write!(f, "<{:?} Arrow Key>", self),
            Key::Enter
            | Key::Tab
            | Key::Backspace
            | Key::Esc
            | Key::Ins
            | Key::Delete
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<crossterm::event::KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        // let KeyEvent {
        //     code,
        //     modifiers,
        //     kind,
        //     state,
        // } = event;
        use crossterm::event;
        match event {
            KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,
            KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => Key::Backspace,
            KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            KeyEvent {
                code: event::KeyCode::Home,
                ..
            } => Key::Home,
            KeyEvent {
                code: event::KeyCode::End,
                ..
            } => Key::End,
            event::KeyEvent {
                code: event::KeyCode::PageUp,
                ..
            } => Key::PageUp,
            event::KeyEvent {
                code: event::KeyCode::PageDown,
                ..
            } => Key::PageDown,
            event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            } => Key::Delete,
            event::KeyEvent {
                code: event::KeyCode::Insert,
                ..
            } => Key::Ins,
            event::KeyEvent {
                code: event::KeyCode::F(n),
                ..
            } => Key::F(n),
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,

            event::KeyEvent {
                code: event::KeyCode::Tab,
                modifiers: event::KeyModifiers::SHIFT,
                ..
            } => Key::ShiftTab,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,

            // First check for char + modifier
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}

// impl From<crossterm::event::KeyEvent> for Key {
//     fn from(key_event: crossterm::event::KeyEvent) -> Self {
//         let KeyEvent {
//             code, modifiers, ..
//         } = key_event;
//         use crossterm::event::KeyCode as RKey;
//         match (code, modifiers) {
//             (RKey::Esc, _) => Key::Esc,
//             (RKey::Backspace, _) => Key::Backspace,
//             (RKey::Left, _) => Key::Left,
//             (RKey::Right, _) => Key::Right,
//             (RKey::Up, _) => Key::Up,
//             (RKey::Down, _) => Key::Down,
//             (RKey::Home, _) => Key::Home,
//             (RKey::End, _) => Key::End,
//             // KeyEvent {
//             //     code: event::KeyCode::Delete,
//             //     ..
//             // } => Key::Delete,
//             // KeyEvent {
//             //     code: event::KeyCode::Insert,
//             //     ..
//             // } => Key::Ins,
//             // KeyEvent {
//             //     code: event::KeyCode::F(n),
//             //     ..
//             // } => Key::F(n),
//             // KeyEvent {
//             //     code: event::KeyCode::Enter,
//             //     ..
//             // } => Key::Enter,
//             // KeyEvent {
//             //     code: tuirealm::event::Key::Tab,
//             //     ..
//             // } => Key::Tab,
//
//             // First check for char + modifier
//             (RKey::Char(c), KeyModifiers::ALT) => Key::Alt(c),
//             (RKey::Char(c), KeyModifiers::CONTROL) => Key::Ctrl(c),
//             (RKey::Char(c), KeyModifiers::NONE) => Key::Char(c),
//             (_, _) => Key::Unknown,
//         }
//     }
// }
