extern crate libthyme;
extern crate ui;

use libthyme::game::*;
use ui::{Action,UI};
use ui::renderer::{initialize_screen,get_action,redraw,cleanup};

/// Run loop of the thyme game, which interprets key presses and processes
/// input by the user.
pub fn main() {
    let mut ui = &mut UI::new();
    let mut game = &mut Game::new();
    let mut hand = None;
    initialize_screen();
    redraw(ui, game, false);
    loop {
        let mut refresh = false;
        let action = get_action();
        match action {
            Action::CursorUp
            | Action::CursorDown
            | Action::CursorRight
            | Action::CursorLeft =>    ui.move_cursor(action),
            Action::Play =>            play_hand(hand, game, ui),
            Action::ToggleSelection => hand = update_selection(game, ui),
            Action::Quit => { break },
            Action::Help => {},
            Action::Hint => {},
            Action::Resize => refresh = true,
            _ => ui.message = "Press 'Q' to quit".to_string()
        }
        redraw(ui, game, refresh);
    }
    cleanup();
}

fn play_hand(hand: Option<MoveType>, game: &mut Game, ui: &mut UI) {
    if hand.is_some() {
        let result = game.play(hand.unwrap(), &ui.selection);
        if result.is_ok() {
            ui.selection.clear();
            ui.message = "".to_string();
        } else {
            ui.message = error_message(result.err().unwrap())
        }
    }
}

fn update_selection(game: &mut Game, ui: &mut UI) -> Option<MoveType> {
    if game.board.top(ui.cursor_position).is_some() {
        ui.toggle_selection();
    }
    let check = game.check(&ui.selection);
    if check.is_ok() {
        return Some(check.ok().unwrap().hand)
    } else {
        ui.message = error_message(check.err().unwrap());
        return None;
    }
}

fn error_message(code: MoveError) -> String {
    return match code {
        MoveError::InvalidMove => "This selection is not a sage hand",
        MoveError::NeedMultipleRows => "A hand must be played from multiple rows",
        MoveError::NoMovesRemain => "Game Over - No moves left",
    }.to_string()
}
