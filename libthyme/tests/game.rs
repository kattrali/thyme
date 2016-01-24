extern crate cards;
extern crate libthyme;

use libthyme::game::*;
use libthyme::board::*;
use libthyme::score::*;
use cards::card::{Card, Suit, Value};

#[test]
fn check_straight_flush() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Clubs },
        Card { value: Value::Nine, suit: Suit::Clubs },
        Card { value: Value::Six, suit: Suit::Clubs },
        Card { value: Value::Eight, suit: Suit::Clubs },
        Card { value: Value::Seven, suit: Suit::Clubs }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::StraightFlush);
}

#[test]
fn check_four_of_a_kind() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Clubs },
        Card { value: Value::Five, suit: Suit::Hearts },
        Card { value: Value::Five, suit: Suit::Spades },
        Card { value: Value::Five, suit: Suit::Diamonds }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::FourOfAKind);
}

#[test]
fn check_full_house() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Hearts },
        Card { value: Value::Five, suit: Suit::Diamonds },
        Card { value: Value::Five, suit: Suit::Clubs },
        Card { value: Value::Eight, suit: Suit::Diamonds },
        Card { value: Value::Eight, suit: Suit::Clubs }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::FullHouse);
}

#[test]
fn check_flush() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Diamonds },
        Card { value: Value::Six, suit: Suit::Diamonds },
        Card { value: Value::Ace, suit: Suit::Diamonds },
        Card { value: Value::Eight, suit: Suit::Diamonds },
        Card { value: Value::Jack, suit: Suit::Diamonds }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::Flush);
}

#[test]
fn check_five_card_straight() {
    let check = check_game(vec![
        Card { value: Value::Seven, suit: Suit::Diamonds },
        Card { value: Value::Six, suit: Suit::Diamonds },
        Card { value: Value::Eight, suit: Suit::Clubs },
        Card { value: Value::Nine, suit: Suit::Diamonds },
        Card { value: Value::Ten, suit: Suit::Diamonds }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::FiveCardStraight);
}

#[test]
fn check_three_of_a_kind() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Hearts },
        Card { value: Value::Five, suit: Suit::Diamonds },
        Card { value: Value::Five, suit: Suit::Clubs }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::ThreeOfAKind);
}

#[test]
fn check_three_card_straight() {
    let check = check_game(vec![
        Card { value: Value::Five, suit: Suit::Hearts },
        Card { value: Value::Four, suit: Suit::Diamonds },
        Card { value: Value::Six, suit: Suit::Clubs }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::ThreeCardStraight);
}

#[test]
fn check_pair() {
    let check = check_game(vec![
        Card { value: Value::Six, suit: Suit::Hearts },
        Card { value: Value::Six, suit: Suit::Clubs }], 0);
    assert_eq!(check.ok().unwrap(), MoveType::Pair);
}

#[test]
fn check_trash() {
    let check = check_game(vec![
        Card { value: Value::Six, suit: Suit::Clubs }], 1);
    assert_eq!(check.ok().unwrap(), MoveType::Trash);
}

#[test]
fn check_no_trash() {
    let check = check_game(vec![
        Card { value: Value::Six, suit: Suit::Clubs }], 0);
    assert_eq!(check.err().unwrap(), MoveError::NoDiscardsRemain);
}

#[test]
fn check_invalid() {
    let check = check_game(vec![
        Card { value: Value::Six, suit: Suit::Hearts },
        Card { value: Value::Eight, suit: Suit::Clubs }], 0);
    assert_eq!(check.err().unwrap(), MoveError::InvalidMove);
}

#[test]
fn test_moves_remain_with_cards() {
    assert!(check_moves(vec![
        Card { value: Value::Six, suit: Suit::Hearts },
        Card { value: Value::Six, suit: Suit::Clubs },
    ], 0));
}

#[test]
fn test_moves_remain_with_discards() {
    assert!(check_moves(vec![Card { value: Value::Six, suit: Suit::Hearts }], 1));
}

#[test]
fn test_no_moves_remain_with_cards() {
    assert!(!check_moves(vec![Card { value: Value::Six, suit: Suit::Hearts }], 0));
}

fn check_moves(cards: Vec<Card>, discards: i32) -> bool {
    let mut positions = vec![
        Position { x: HPosition::Left, y: VPosition::Top },
        Position { x: HPosition::Center, y: VPosition::Middle },
        Position { x: HPosition::Right, y: VPosition::Top },
        Position { x: HPosition::Left, y: VPosition::Middle },
        Position { x: HPosition::Center, y: VPosition::Top }];
    positions.truncate(cards.len());
    return setup_game::<StandardScorer>(cards, &positions, discards).moves_remaining()
}

fn check_game(cards: Vec<Card>, discards: i32) -> Result<MoveType, MoveError> {
    let mut positions = vec![
        Position { x: HPosition::Left, y: VPosition::Top },
        Position { x: HPosition::Center, y: VPosition::Middle },
        Position { x: HPosition::Right, y: VPosition::Top },
        Position { x: HPosition::Left, y: VPosition::Middle },
        Position { x: HPosition::Center, y: VPosition::Top }];
    positions.truncate(cards.len());
    return setup_game::<StandardScorer>(cards, &positions, discards).check(&positions);
}

fn setup_game<T: Scorer>(cards: Vec<Card>, positions: &Vec<Position>, discards: i32) -> Game<T> {
    let mut stacks = Vec::new();
    for index in 0..cards.len() {
        stacks.push(Stack { cards: vec![cards[index]], position: positions[index] });
    }
    stacks.push(Stack {
        cards: vec![Card { value: Value::Ace, suit: Suit::Spades }],
        position: Position { x: HPosition::Left, y: VPosition::Bottom } });
    stacks.push(Stack {
        cards: vec![Card { value: Value::Jack, suit: Suit::Spades }],
        position: Position { x: HPosition::Center, y: VPosition::Bottom } });
    stacks.push(Stack {
        cards: vec![Card { value: Value::King, suit: Suit::Spades }],
        position: Position { x: HPosition::Right, y: VPosition::Bottom } });
    stacks.push(Stack {
        cards: vec![Card { value: Value::Seven, suit: Suit::Spades }],
        position: Position { x: HPosition::Right, y: VPosition::Middle } });
    let lucky_card = Card { value: Value::King, suit: Suit::Hearts };
    let board = Board { stacks: stacks, lucky_card: lucky_card };
    return Game {
        board: board,
        discards_allowed: discards,
        discards_allowed_max: discards,
        scorer: Scorer::new(lucky_card) };
}

