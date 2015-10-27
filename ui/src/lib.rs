pub mod renderer;

extern crate libthyme;

use libthyme::board::{Position,HPosition,VPosition};

pub struct UI {
    pub cursor_position: Position,
    pub message: String,
    pub selection: Vec<Position>,
}

pub enum Action {
    CursorDown,
    CursorLeft,
    CursorRight,
    CursorUp,
    Help,
    Hint,
    Play,
    Quit,
    Resize,
    ToggleSelection,
    Unknown,
}

impl UI {

    /// Create a new UI with the cursor in the top left corner and no selection
    pub fn new() -> UI {
        UI {
            cursor_position: Position {
                x: HPosition::Left,
                y: VPosition::Top  },
            message: "".to_string(),
            selection: vec![]
        }
    }

    /// Toggle the selection of the cursor position
    pub fn toggle_selection(&mut self) {
        let pos = self.cursor_position;
        if self.selection.contains(&pos) {
            self.selection.retain(|s| *s != pos)
        } else {
            self.selection.push(self.cursor_position)
        }
    }

    /// Move the cursor position using a cursor action
    pub fn move_cursor(&mut self, direction: Action) {
        match direction {
            Action::CursorUp => {
                match self.cursor_position.y {
                    VPosition::Bottom => self.cursor_position.y = VPosition::Middle,
                    VPosition::Middle => self.cursor_position.y = VPosition::Top,
                    _ => {}
                }
            },
            Action::CursorDown => {
                match self.cursor_position.y {
                    VPosition::Top => self.cursor_position.y = VPosition::Middle,
                    VPosition::Middle => self.cursor_position.y = VPosition::Bottom,
                    _ => {}
                }
            },
            Action::CursorLeft => {
                match self.cursor_position.x {
                    HPosition::Right => self.cursor_position.x = HPosition::Center,
                    HPosition::Center => self.cursor_position.x = HPosition::Left,
                    _ => {}
                }
            },
            Action::CursorRight => {
                match self.cursor_position.x {
                    HPosition::Left => self.cursor_position.x = HPosition::Center,
                    HPosition::Center => self.cursor_position.x = HPosition::Right,
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

