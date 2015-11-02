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
            if game.moves_remaining() {
                ui.message = play_message(hand.unwrap())
            } else if game.board.positions_remaining().len() > 0 {
                ui.message = error_message(MoveError::NoMovesRemain)
            } else {
                ui.message = success_message();
            }
        } else {
            ui.message = error_message(result.err().unwrap())
        }
    }
}

fn update_selection(game: &mut Game, ui: &mut UI) -> Option<MoveType> {
    if !game.moves_remaining() {
        ui.message = error_message(MoveError::NoMovesRemain);
        return None;
    }
    if game.board.top(ui.cursor_position).is_some() {
        ui.toggle_selection();
    }
    if ui.selection.len() == 0 {
        ui.message.clear();
        return None;
    }
    let check = game.check(&ui.selection);
    if check.is_ok() {
        let hand = check.ok().unwrap();
        ui.message = check_message(hand, game);
        return Some(hand)
    } else {
        ui.message = error_message(check.err().unwrap());
        return None;
    }
}

fn success_message() -> String {
    return "You WON!".to_string();
}

fn error_message(code: MoveError) -> String {
    return match code {
        MoveError::InvalidMove => "This selection is not a hand",
        MoveError::InvalidHand => "This selection does not match the hand",
        MoveError::NeedMultipleRows => "A hand must be played from multiple rows",
        MoveError::NoMovesRemain => "Game Over - No moves left",
        MoveError::NoDiscardsRemain => "No discards remain",
        MoveError::TwoPairIsInvalid => "Two pair is not a hand",
    }.to_string()
}

fn check_message(code: MoveType, game: &Game) -> String {
    if code == MoveType::Trash {
        return format!("Press return to discard this card. ({}/{} remaining)",
        game.discards_allowed, game.discards_allowed_max);
    }
    return format!("Press return to play '{}'", hand_message(code))
}

fn play_message(code: MoveType) -> String {
    return format!("Played '{}'", hand_message(code))
}

fn hand_message(code: MoveType) -> String {
    return match code {
        MoveType::StraightFlush => "Straight Flush",
        MoveType::FourOfAKind => "Four of a Kind",
        MoveType::Flush => "Flush",
        MoveType::FullHouse => "Full House",
        MoveType::FiveCardStraight => "Five-card Straight",
        MoveType::ThreeOfAKind => "Three of a Kind",
        MoveType::ThreeCardStraight => "Three-card Straight",
        MoveType::Pair => "Pair",
        MoveType::Trash => "Discard",
    }.to_string()
}
