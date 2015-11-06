extern crate cards;
extern crate libthyme;

use cards::card::{Card, Suit, Value};
use libthyme::score::*;

#[test]
fn starts_with_0_score() {
    assert_eq!(0, scorer().running_total());
}

