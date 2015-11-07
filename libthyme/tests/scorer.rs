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
fn doubles_value_for_lucky_suit() {
    let mut scorer = scorer();
    let play = Play {
        hand: MoveType::ThreeOfAKind,
        cleared_positions: vec![],
        cards: vec![Card { value: Value::King, suit: Suit::Clubs }] };
    let score = scorer.check_play(play.clone());
    scorer.add_play(play);
    assert_eq!(30, score.value);
    assert_eq!(2, score.multiplier);
    assert_eq!(60, scorer.score(false));
}

#[test]
fn awards_30_for_three_of_a_kind() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::ThreeOfAKind));
    scorer.add_play(play(MoveType::ThreeOfAKind));
    assert_eq!(30, score.value);
    assert_eq!(30, scorer.score(false));
}

#[test]
fn awards_20_for_three_card_straight() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::ThreeCardStraight));
    scorer.add_play(play(MoveType::ThreeCardStraight));
    assert_eq!(20, score.value);
    assert_eq!(20, scorer.score(false));
}

#[test]
fn awards_10_for_pair() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::Pair));
    scorer.add_play(play(MoveType::Pair));
    assert_eq!(10, score.value);
    assert_eq!(10, scorer.score(false));
}

#[test]
fn awards_0_for_trash() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::Trash));
    scorer.add_play(play(MoveType::Trash));
    assert_eq!(0, score.value);
    assert_eq!(0, scorer.score(false));
}

#[test]
fn awards_50_for_five_card_straight() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::FiveCardStraight));
    scorer.add_play(play(MoveType::FiveCardStraight));
    assert_eq!(50, score.value);
    assert_eq!(50, scorer.score(false));
}

#[test]
fn awards_70_for_full_house() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::FullHouse));
    scorer.add_play(play(MoveType::FullHouse));
    assert_eq!(70, score.value);
    assert_eq!(70, scorer.score(false));
}

#[test]
fn awards_90_for_flush() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::Flush));
    scorer.add_play(play(MoveType::Flush));
    assert_eq!(90, score.value);
    assert_eq!(90, scorer.score(false));
}

#[test]
fn awards_100_for_four_of_a_kind() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::FourOfAKind));
    scorer.add_play(play(MoveType::FourOfAKind));
    assert_eq!(100, score.value);
    assert_eq!(100, scorer.score(false));
}

#[test]
fn awards_150_for_straight_flush() {
    let mut scorer = scorer();
    let score = scorer.check_play(play(MoveType::StraightFlush));
    scorer.add_play(play(MoveType::StraightFlush));
    assert_eq!(150, score.value);
    assert_eq!(150, scorer.score(false));
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

fn play(hand: MoveType) -> Play {
    Play { hand: hand, cleared_positions: vec![], cards: vec![] }
}

