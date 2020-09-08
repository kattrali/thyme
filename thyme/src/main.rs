extern crate libthyme;
extern crate ui;

use libthyme::game::*;
use libthyme::score::{Play,Scorer,StandardScorer};
use ui::{Action,UI};
use ui::renderer::{initialize_screen,get_action,redraw,cleanup};

/// Run loop of the thyme game, which interprets key presses and processes
/// input by the user.
pub fn main() {
    let mut ui = &mut UI::new();
    let game = &mut Game::<StandardScorer>::new();
    let mut hand = None;
    initialize_screen();
    redraw(ui, game, true);
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

/// Play the cards in the selected positions if possible, and if so then clear
/// the selection.
/// Prints a message reflecting the current game state.
fn play_hand<T: Scorer>(hand: Option<MoveType>, game: &mut Game<T>, ui: &mut UI) {
    if hand.is_some() {
        let result = game.play(hand.unwrap(), &ui.selection);
        if result.is_ok() {
            game.scorer.add_play(result.ok().unwrap());
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

/// Toggle the selection of the cursor-selected card, if cards remain in that
/// position and the game has not ended.
/// Prints a message reflecting the current game state.
fn update_selection<T: Scorer>(game: &mut Game<T>, ui: &mut UI) -> Option<MoveType> {
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
        ui.message = check_message(hand, ui, game);
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

fn check_message<T: Scorer>(hand: MoveType, ui: &UI, game: &mut Game<T>) -> String {
    if hand == MoveType::Trash {
        return format!("Press return to discard this card.");
    }
    let cleared = ui.selection.iter().filter(|p| game.board.count_cards(**p) == 1).map(|p| *p).collect();
    let score = game.scorer.check_play(Play {
        cards: game.board.peek(&ui.selection).unwrap(),
        cleared_positions: cleared,
        hand: hand
    });
    return format!("Press return to play '{}' (+{} x{})",
                   hand_message(hand),
                   score.value,
                   score.multiplier)
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
