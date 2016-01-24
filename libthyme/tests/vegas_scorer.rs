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
fn awards_40_for_straight_flush() {
    check_score(MoveType::StraightFlush, 40);
}

#[test]
fn awards_20_for_four_of_a_kind() {
    check_score(MoveType::FourOfAKind, 20);
}

#[test]
fn awards_10_for_flush() {
    check_score(MoveType::Flush, 10);
}

#[test]
fn awards_8_for_full_house() {
    check_score(MoveType::FullHouse, 8);
}

#[test]
fn awards_5_for_five_card_straight() {
    check_score(MoveType::FiveCardStraight, 5);
}

#[test]
fn awards_3_for_three_of_a_kind() {
    check_score(MoveType::ThreeOfAKind, 3);
}

#[test]
fn awards_2_for_three_card_straight() {
    check_score(MoveType::ThreeCardStraight, 2);
}

#[test]
fn awards_1_for_pair() {
    check_score(MoveType::Pair, 1);
}

#[test]
fn awards_0_for_trash() {
    check_score(MoveType::Trash, 0);
}

#[test]
fn awards_0_without_lucky_card() {
    for hand in all_hands() {
        check_unlucky_hand(hand);
    }
}

#[test]
fn computes_final_score_of_0_without_cleared_stacks() {
    for hand in all_hands() {
        let mut scorer = scorer();
        scorer.add_play(play(hand));
        assert_eq!(0, scorer.score(true));
    }
}

#[test]
fn computes_final_score_of_0_with_bottom_cleared_stacks() {
    let mut scorer = scorer();
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        for hand in all_hands() {
            let cards = vec![lucky_card()];
            let cleared = vec![Position { x: x, y: VPosition::Bottom }];
            scorer.add_play(Play {
                hand: hand, cleared_positions: cleared, cards: cards });
            assert_eq!(0, scorer.score(true));
        }
    }
}

#[test]
fn quarters_final_score_with_middle_cleared_stacks() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        let mut scorer = scorer();
        let cleared = vec![Position { x: x, y: VPosition::Middle }];
        scorer.add_play(Play {
            hand: MoveType::StraightFlush,
            cleared_positions: cleared, cards: vec![lucky_card()] });
        assert_eq!(10, scorer.score(true));
    }
}

#[test]
fn three_quarters_final_score_with_top_cleared_stacks() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        let mut scorer = scorer();
        let cards = vec![lucky_card()];
        let cleared = vec![Position { x: x, y: VPosition::Top }];
        scorer.add_play(Play {
            hand: MoveType::StraightFlush,
            cleared_positions: cleared, cards: cards });
        assert_eq!(30, scorer.score(true));
    }
}

#[test]
fn aggregates_multiplier_for_cleared_stacks() {
    let positions = vec![HPosition::Left, HPosition::Center, HPosition::Right];
    for x in positions {
        for hand in all_hands() {
            let mut scorer = scorer();
            let score = scorer.check_play(play(hand));
            let cards = vec![lucky_card()];
            let cleared = vec![
                Position { x: x, y: VPosition::Middle },
                Position { x: x, y: VPosition::Top }];
            scorer.add_play(Play {
                hand: hand, cleared_positions: cleared, cards: cards });
            assert_eq!(score.value, scorer.score(true));
        }
    }
}

fn all_hands() -> Vec<MoveType> {
    return vec![
        MoveType::FullHouse, MoveType::ThreeOfAKind, MoveType::Pair,
        MoveType::FourOfAKind, MoveType::FiveCardStraight, MoveType::Trash,
        MoveType::ThreeCardStraight, MoveType::Flush, MoveType::StraightFlush
    ];
}

fn lucky_card() -> Card {
    return Card { value: Value::Seven, suit: Suit::Clubs };
}

fn unlucky_card() -> Card {
    return Card { value: Value::Seven, suit: Suit::Spades };
}

fn scorer() -> VegasScorer {
    return VegasScorer::new(lucky_card());
}

fn play(hand: MoveType) -> Play {
    Play { hand: hand, cleared_positions: vec![], cards: vec![lucky_card()] }
}

fn check_score(hand: MoveType, expected: i32) {
    let mut scorer = scorer();
    let score = scorer.check_play(play(hand));
    scorer.add_play(play(hand));
    assert_eq!(expected, score.value);
    assert_eq!(expected, scorer.score(false));
}

fn unlucky_play(hand: MoveType) -> Play {
    return Play {
        hand: hand, cleared_positions: vec![], cards: vec![unlucky_card()] };
}

fn check_unlucky_hand(hand: MoveType) {
    let mut scorer = scorer();
    let score = scorer.check_play(unlucky_play(hand));
    scorer.add_play(unlucky_play(hand));
    assert_eq!(0, score.value);
    assert_eq!(0, scorer.score(false));
}
