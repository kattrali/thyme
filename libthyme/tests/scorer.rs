extern crate cards;
extern crate libthyme;

use cards::card::{Card, Suit, Value};
use libthyme::score::*;
use libthyme::board::{Position,HPosition,VPosition};
use libthyme::game::MoveType;

#[test]
fn starts_with_0_score() {
    assert_eq!(0, scorer().score(false));
}


#[test]
fn formats_scores() {
    assert_eq!("0", scorer().format_as_score(0));
    assert_eq!("10", scorer().format_as_score(10));
}

#[test]
fn awards_150_for_clearing_top_position() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        let scorer = scorer();
        let position = Position { x: x, y: VPosition::Top  };
        assert_eq!(150, scorer.bonus(position));
        assert_eq!(150, scorer.check_play(Play {
            cards: vec![], hand: MoveType::FiveCardStraight, cleared_positions: vec![position]
        }).bonus);
    }
}

#[test]
fn awards_100_for_clearing_middle_position() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        let scorer = scorer();
        let position = Position { x: x, y: VPosition::Middle  };
        assert_eq!(100, scorer.bonus(position));
        assert_eq!(100, scorer.check_play(Play {
            cards: vec![], hand: MoveType::FiveCardStraight, cleared_positions: vec![position]
        }).bonus);
    }
}

#[test]
fn awards_50_for_clearing_bottom_position() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        let scorer = scorer();
        let position = Position { x: x, y: VPosition::Bottom  };
        assert_eq!(50, scorer.bonus(position));
        assert_eq!(50, scorer.check_play(Play {
            cards: vec![], hand: MoveType::FiveCardStraight, cleared_positions: vec![position]
        }).bonus);
    }
}

fn scorer() -> StandardScorer {
    let card = Card { value: Value::Seven, suit: Suit::Clubs };
    return StandardScorer::new(card);
}
