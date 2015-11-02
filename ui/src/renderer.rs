extern crate cards;
extern crate libthyme;
extern crate ncurses;

use super::{Action,UI};
use libthyme::board::{Position,HPosition,VPosition};
use libthyme::game::Game;

const CARD_WIDTH: i32 = 7;
const CARD_HEIGHT: i32 = 5;
const CARD_MARGIN: i32 = 2;
const BOARD_MARGIN: i32 = 1;
const STATUS_HEIGHT: i32 = 1;

const CARD_COLOR_BLACK: i16 = 1; // black on white
const CARD_COLOR_RED: i16 = 2; // red on white
const CARD_COLOR_EMPTY: i16 = 4; // white on black
const COLOR_SELECTED: i16 = 5; // yellow on black

/// Set up the UI
pub fn initialize_screen() {
    ncurses::setlocale(ncurses::LcCategory::ctype, "");
    ncurses::initscr();
    ncurses::noecho();
    ncurses::start_color();
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::init_pair(CARD_COLOR_BLACK, ncurses::COLOR_BLACK, ncurses::COLOR_WHITE);
    ncurses::init_pair(CARD_COLOR_RED, ncurses::COLOR_RED, ncurses::COLOR_WHITE);
    ncurses::init_pair(CARD_COLOR_EMPTY, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
    ncurses::init_pair(COLOR_SELECTED, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK);
}

/// Redraw a UI in the current screen
pub fn redraw(ui: &UI, game: &mut Game, refresh: bool) {
    if refresh {
        ncurses::clear();
    }
    if validate_screen_size() {
        write_title(game);
        draw_cards(ui, game);
        write_message(&ui.message);
    }
    ncurses::refresh();
}

/// Tear down the UI
pub fn cleanup() {
    ncurses::endwin();
}

/// Process input from the user
///
/// Known inputs:
/// - Arrow keys: Move cursor between various positions on the board
/// - Q: Quit game
/// - H: Hint (unimplemented)
/// - ?: Help
/// - Space: Toggle position selection
/// - Return: Play move, clear selection
pub fn get_action() -> Action {
    return match ncurses::getch() {
        ncurses::KEY_LEFT => Action::CursorLeft,
        ncurses::KEY_RIGHT => Action::CursorRight,
        ncurses::KEY_UP => Action::CursorUp,
        ncurses::KEY_DOWN => Action::CursorDown,
        ncurses::KEY_RESIZE => Action::Resize,
        ncurses::KEY_ENTER | 13 | 10 => Action::Play,
        32 =>  Action::ToggleSelection, // Space
        113 => Action::Quit, // Q
        104 => Action::Hint, // H
        63 =>  Action::Help, // ?
        _ =>   Action::Unknown
    }
}

/// Check that the content can fit
fn validate_screen_size() -> bool {
    let min_height = BOARD_MARGIN*2 + CARD_MARGIN*6 + STATUS_HEIGHT + CARD_HEIGHT*3;
    if ncurses::LINES < min_height || ncurses::COLS < 50 {
        write_message(&format!("Please resize your terminal to be at least 50x{}", min_height));
        return false
    }
    return true
}

/// Print the game title and status info
fn write_title(game: &Game) {
    ncurses::attron(ncurses::A_BOLD());
    ncurses::mvprintw(0, 0, "Thyme");
    ncurses::attroff(ncurses::A_BOLD());
    let (_, suit) = layout_suit(game.board.lucky_card);
    ncurses::printw(&format!(" - Lucky Suit: {}", suit));
    ncurses::clrtoeol();
}

/// Print the message at the bottom of the window
fn write_message(message: &str) {
    ncurses::mvprintw(ncurses::LINES - 1, 0, message);
    ncurses::clrtoeol();
}

fn draw_cards(ui: &UI, game: &mut Game) {
    for position in game.board.positions() {
        let card = game.board.top(position);
        if card.is_some() {
            draw_card(position, card.unwrap());
        } else {
            draw_empty(position);
        }
        let (x, y) = card_location(position);
        toggle_highlight_card(x, y, ui.selection.contains(&position));
        if position == ui.cursor_position {
            let offset = (CARD_WIDTH as f32 /2.0).floor() as i32;
            ncurses::mvprintw(y + CARD_HEIGHT, x + offset, "*");
        }
    }
}

/// Draw a card on the board at a position
fn draw_card(position: Position, card: cards::card::Card) {
    let (x, y) = card_location(position);
    let (color, suit) = layout_suit(card);
    let value = layout_value(card);
    let black = ncurses::COLOR_PAIR(CARD_COLOR_BLACK);
    ncurses::attron(black);
    ncurses::mvprintw(y, x, &value);
    ncurses::attroff(black);
    ncurses::attron(color);
    ncurses::mvprintw(y, x + value.len() as i32, &suit);
    ncurses::attroff(color);
    let spacing = CARD_WIDTH - value.len() as i32 - 1;
    printw_repeat(" ", spacing, black);
    for i in 1..CARD_HEIGHT - 1 {
        ncurses::mv(y + i, x);
        printw_repeat(" ", CARD_WIDTH, black);
    }
    ncurses::mv(y + CARD_HEIGHT - 1, x);
    printw_repeat(" ", spacing, black);
    ncurses::attron(black);
    ncurses::printw(&value);
    ncurses::attroff(black);
    ncurses::attron(color);
    ncurses::printw(&suit);
    ncurses::attroff(color);
}

/// Draw empty slot for a card
fn draw_empty(position: Position) {
    let color = ncurses::COLOR_PAIR(CARD_COLOR_EMPTY);
    let (x, y) = card_location(position);
    ncurses::attron(color);
    ncurses::mvprintw(y, x, "┌");
    printw_repeat("─", CARD_WIDTH - 2, color);
    ncurses::mvprintw(y, x + CARD_WIDTH - 1, "┐");
    ncurses::attroff(color);
    for i in 1..CARD_HEIGHT - 1 {
        ncurses::attron(color);
        ncurses::mvprintw(y + i, x , "│");
        printw_repeat(" ", CARD_WIDTH - 2, color);
        ncurses::attroff(color);
        ncurses::attron(color);
        ncurses::mvprintw(y + i, x + CARD_WIDTH - 1, "│");
        ncurses::attroff(color);
    }
    ncurses::attron(color);
    ncurses::mvprintw(y + CARD_HEIGHT - 1, x, "└");
    printw_repeat("─", CARD_WIDTH - 2, color);
    ncurses::mvprintw(y + CARD_HEIGHT - 1, x + CARD_WIDTH - 1, "┘");
    ncurses::attroff(color);
}

fn printw_repeat(content: &str, len: i32, color: u64) {
    ncurses::attron(color);
    for _ in 0..len {
       ncurses::printw(content);
    }
    ncurses::attroff(color);
}

fn toggle_highlight_card(x: i32, y: i32, on: bool) {
    let color = ncurses::COLOR_PAIR(COLOR_SELECTED);
    ncurses::attron(color);
    ncurses::mvprintw(y - 1, x - 1, if on {"┌"} else {" "});
    ncurses::attroff(color);
    ncurses::mv(y - 1, x);
    printw_repeat(if on {"─"} else {" "}, CARD_WIDTH, color);
    ncurses::mv(y + CARD_HEIGHT, x);
    printw_repeat(if on {"─"} else {" "}, CARD_WIDTH, color);
    ncurses::attron(color);
    ncurses::mvprintw(y - 1, x + CARD_WIDTH, if on {"┐"} else {" "});
    for i in 0..CARD_HEIGHT {
        ncurses::mvprintw(y + i, x - 1, if on {"│"} else {" "});
        ncurses::mvprintw(y + i, x + CARD_WIDTH, if on {"│"} else {" "});
    }
    ncurses::mvprintw(y + CARD_HEIGHT, x - 1, if on {"└"} else {" "});
    ncurses::mvprintw(y + CARD_HEIGHT, x + CARD_WIDTH, if on {"┘"} else {" "});
    ncurses::attroff(color);
}

fn layout_suit(card: cards::card::Card) -> (u64, String) {
    let black = ncurses::COLOR_PAIR(CARD_COLOR_BLACK);
    let red = ncurses::COLOR_PAIR(CARD_COLOR_RED);
    return match card.suit {
        cards::card::Suit::Diamonds => (red, "\u{2666}".to_string()),
        cards::card::Suit::Clubs => (black, "\u{2663}".to_string()),
        cards::card::Suit::Spades => (black, "\u{2660}".to_string()),
        cards::card::Suit::Hearts => (red, "\u{2665}".to_string()),
    }
}

fn layout_value(card: cards::card::Card) -> String {
    return match card.value {
        cards::card::Value::Ace => "A",
        cards::card::Value::Two => "2",
        cards::card::Value::Three => "3",
        cards::card::Value::Four => "4",
        cards::card::Value::Five => "5",
        cards::card::Value::Six => "6",
        cards::card::Value::Seven => "7",
        cards::card::Value::Eight => "8",
        cards::card::Value::Nine => "9",
        cards::card::Value::Ten => "10",
        cards::card::Value::Jack => "J",
        cards::card::Value::Queen => "Q",
        cards::card::Value::King => "K",
    }.to_string()
}

/// Location (x, y) for a card position
fn card_location(position: Position) -> (i32, i32) {
    let left = BOARD_MARGIN + CARD_MARGIN;
    let center = BOARD_MARGIN + CARD_MARGIN*3 + CARD_WIDTH;
    let right = BOARD_MARGIN + CARD_MARGIN*5 + CARD_WIDTH*2;
    let top = BOARD_MARGIN + CARD_MARGIN;
    let middle = BOARD_MARGIN + CARD_MARGIN*3 + CARD_HEIGHT;
    let bottom = BOARD_MARGIN + CARD_MARGIN*5 + CARD_HEIGHT*2;
    match position {
        Position { x: HPosition::Left, y: VPosition::Top } => (left, top),
        Position { x: HPosition::Left, y: VPosition::Middle } => (left, middle),
        Position { x: HPosition::Left, y: VPosition::Bottom } => (left, bottom),
        Position { x: HPosition::Center, y: VPosition::Top } => (center, top),
        Position { x: HPosition::Center, y: VPosition::Middle } => (center, middle),
        Position { x: HPosition::Center, y: VPosition::Bottom } => (center, bottom),
        Position { x: HPosition::Right, y: VPosition::Top } => (right, top),
        Position { x: HPosition::Right, y: VPosition::Middle } => (right, middle),
        Position { x: HPosition::Right, y: VPosition::Bottom } => (right, bottom),
    }
}
