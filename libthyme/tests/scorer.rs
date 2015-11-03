extern crate cards;
extern crate libthyme;

use cards::card::{Card, Suit, Value};
use libthyme::score::*;

#[test]
fn standard_starts_with_0_score() {
    let card = Card { value: Value::Seven, suit: Suit::Clubs };
    assert_eq!("0", StandardScorer::new(card).running_total());
}

