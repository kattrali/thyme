extern crate cards;
extern crate libthyme;

use cards::card::{Card, Suit, Value};
use libthyme::card::*;

#[test]
fn single_card_is_not_same_suit() {
    assert!(!is_same_suit(&vec![Card { value: Value::Ace, suit: Suit::Spades }]))
}

#[test]
fn one_suit_is_same_suit() {
    let hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Three, suit: Suit::Spades },
        Card { value: Value::Jack, suit: Suit::Spades }];
    assert!(is_same_suit(&hand))
}

#[test]
fn ace_low_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Three, suit: Suit::Spades },
        Card { value: Value::Two, suit: Suit::Spades }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn ace_high_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Queen, suit: Suit::Spades },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn sorted_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::King, suit: Suit::Spades },
        Card { value: Value::Queen, suit: Suit::Spades },
        Card { value: Value::Jack, suit: Suit::Spades },
        Card { value: Value::Ten, suit: Suit::Spades }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn unsorted_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Queen, suit: Suit::Spades },
        Card { value: Value::King, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Ten, suit: Suit::Spades },
        Card { value: Value::Jack, suit: Suit::Spades }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn varying_suits_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Queen, suit: Suit::Hearts },
        Card { value: Value::King, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Clubs },
        Card { value: Value::Ten, suit: Suit::Clubs },
        Card { value: Value::Jack, suit: Suit::Spades }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn two_cards_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::King, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Clubs }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn three_cards_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Six, suit: Suit::Spades },
        Card { value: Value::Five, suit: Suit::Clubs },
        Card { value: Value::Seven, suit: Suit::Clubs }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn four_cards_is_consecutive() {
    let mut hand = vec![
        Card { value: Value::Four, suit: Suit::Spades },
        Card { value: Value::Seven, suit: Suit::Clubs },
        Card { value: Value::Five, suit: Suit::Diamonds },
        Card { value: Value::Six, suit: Suit::Clubs }];
    assert!(is_consecutive(&mut hand))
}

#[test]
fn empty_is_not_consecutive() {
    let mut hand = Vec::<Card>::new();
    assert!(!is_consecutive(&mut hand))
}

#[test]
fn single_card_is_not_consecutive() {
    let mut hand = vec![Card { value: Value::Ace, suit: Suit::Spades }];
    assert!(!is_consecutive(&mut hand))
}

#[test]
fn wraparound_is_not_consecutive() {
    let mut hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Two, suit: Suit::Spades },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(!is_consecutive(&mut hand))
}

#[test]
fn discontinuous_is_not_consecutive() {
    let mut hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Jack, suit: Suit::Spades },
        Card { value: Value::Three, suit: Suit::Spades },
        Card { value: Value::Four, suit: Suit::Spades },
        Card { value: Value::Five, suit: Suit::Spades }];
    assert!(!is_consecutive(&mut hand))
}

#[test]
fn contains_too_few_of_multiple_is_not_match() {
    let hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Hearts },
        Card { value: Value::Ace, suit: Suit::Clubs },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(!contains_multiple_of_value(&hand, 4));
}

#[test]
fn contains_too_many_of_multiple_is_not_match() {
    let hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Hearts },
        Card { value: Value::Ace, suit: Suit::Clubs },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(!contains_multiple_of_value(&hand, 2));
}

#[test]
fn contains_single_multiple_is_match() {
    let hand = vec![
        Card { value: Value::Eight, suit: Suit::Spades },
        Card { value: Value::Eight, suit: Suit::Hearts },
        Card { value: Value::King, suit: Suit::Clubs },
        Card { value: Value::Three, suit: Suit::Spades }];
    assert!(contains_multiple_of_value(&hand, 2));
}

#[test]
fn contains_multiple_multiples_is_match() {
    let hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Hearts },
        Card { value: Value::King, suit: Suit::Clubs },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(contains_multiple_of_value(&hand, 2));
}

#[test]
fn contains_other_multiple_multiples_is_match() {
    let hand = vec![
        Card { value: Value::Ace, suit: Suit::Spades },
        Card { value: Value::Ace, suit: Suit::Hearts },
        Card { value: Value::Ace, suit: Suit::Clubs },
        Card { value: Value::King, suit: Suit::Clubs },
        Card { value: Value::King, suit: Suit::Spades }];
    assert!(contains_multiple_of_value(&hand, 2));
}
