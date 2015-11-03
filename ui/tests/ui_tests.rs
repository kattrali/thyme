extern crate ui;
extern crate libthyme;

use ui::{UI,Action};
use libthyme::board::{Position,HPosition,VPosition};

#[test]
fn toggle_selection_on() {
    let mut ui = UI::new();
    assert!(!ui.selection.contains(&ui.cursor_position));
    ui.toggle_selection();
    assert!(ui.selection.contains(&ui.cursor_position));
}

#[test]
fn toggle_selection_off() {
    let mut ui = UI::new();
    ui.selection = vec![
        Position { x: HPosition::Center, y: VPosition::Bottom },
        Position { x: HPosition::Right, y: VPosition::Middle },
        Position { x: HPosition::Center, y: VPosition::Top }];
    ui.cursor_position = Position { x: HPosition::Center, y: VPosition::Top };
    ui.toggle_selection();
    assert!(!ui.selection.contains(&ui.cursor_position));
}

#[test]
fn move_cursor_up_from_bottom() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Bottom };
        ui.move_cursor(Action::CursorUp);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Middle });
    }
}

#[test]
fn move_cursor_up_from_middle() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Middle };
        ui.move_cursor(Action::CursorUp);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Top });
    }
}

#[test]
fn move_cursor_up_from_top() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Top };
        ui.move_cursor(Action::CursorUp);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Top });
    }
}

#[test]
fn move_cursor_down_from_top() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Top };
        ui.move_cursor(Action::CursorDown);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Middle });
    }
}

#[test]
fn move_cursor_down_from_middle() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Middle };
        ui.move_cursor(Action::CursorDown);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Bottom });
    }
}

#[test]
fn move_cursor_down_from_bottom() {
    let mut ui = UI::new();
    let hp = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in hp {
        ui.cursor_position = Position { x: x, y: VPosition::Bottom };
        ui.move_cursor(Action::CursorDown);
        assert_eq!(ui.cursor_position, Position { x: x, y: VPosition::Bottom });
    }
}

#[test]
fn move_cursor_left_from_right() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Right, y: y};
        ui.move_cursor(Action::CursorLeft);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Center, y: y });
    }
}

#[test]
fn move_cursor_left_from_center() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Center, y: y};
        ui.move_cursor(Action::CursorLeft);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Left, y: y });
    }
}

#[test]
fn move_cursor_left_from_left() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Left, y: y};
        ui.move_cursor(Action::CursorLeft);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Left, y: y });
    }
}

#[test]
fn move_cursor_right_from_left() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Left, y: y};
        ui.move_cursor(Action::CursorRight);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Center, y: y });
    }
}

#[test]
fn move_cursor_right_from_center() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Center, y: y};
        ui.move_cursor(Action::CursorRight);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Right, y: y });
    }
}

#[test]
fn move_cursor_right_from_right() {
    let mut ui = UI::new();
    let vp = vec![VPosition::Top, VPosition::Middle, VPosition::Bottom];
    for y in vp {
        ui.cursor_position = Position { x: HPosition::Right, y: y};
        ui.move_cursor(Action::CursorRight);
        assert_eq!(ui.cursor_position, Position { x: HPosition::Right, y: y });
    }
}
