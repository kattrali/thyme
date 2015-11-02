extern crate cards;
extern crate libthyme;

use libthyme::board::*;
use cards::card::Card;

#[test]
fn has_all_cards_minus_one() {
    let board = Board::new();
    assert_eq!(51, board.count_all_cards());
    let mut count = 0;
    for stack in &board.stacks {
        count += stack.cards.len();
    }
    assert_eq!(51, count);
}

#[test]
fn has_eight_cards_in_top_left() {
    check_count(HPosition::Left, VPosition::Top, 8)
}

#[test]
fn has_eight_cards_in_top_center() {
    check_count(HPosition::Center, VPosition::Top, 8)
}

#[test]
fn has_eight_cards_in_top_right() {
    check_count(HPosition::Right, VPosition::Top, 8)
}

#[test]
fn has_seven_cards_in_middle_left() {
    check_count(HPosition::Left, VPosition::Middle, 7)
}

#[test]
fn has_six_cards_in_middle_center() {
    check_count(HPosition::Center, VPosition::Middle, 6)
}

#[test]
fn has_five_cards_in_middle_right() {
    check_count(HPosition::Right, VPosition::Middle, 5)
}

#[test]
fn has_four_cards_in_bottom_left() {
    check_count(HPosition::Left, VPosition::Bottom, 4)
}

#[test]
fn has_three_cards_in_bottom_center() {
    check_count(HPosition::Center, VPosition::Bottom, 3)
}

#[test]
fn has_two_cards_in_bottom_right() {
    check_count(HPosition::Right, VPosition::Bottom, 2)
}

#[test]
fn lucky_card_not_in_stacks() {
    let board = Board::new();
    let lucky_card = board.lucky_card;
    assert!(!all_cards(board).contains(&lucky_card))
}

#[test]
fn no_duplicates_in_stacks() {
    let board = Board::new();
    let mut seen = Vec::new();
    for card in all_cards(board) {
        assert!(!seen.contains(&card));
        seen.push(card);
    }
}

#[test]
fn has_all_positions() {
    assert_eq!(Board::new().positions().len(), 9);
}

fn all_cards(mut board: Board) -> Vec<Card> {
    let mut stack = Vec::<Card>::new();
    stack.extend(&mut empty_stack(&mut board, HPosition::Left, VPosition::Top).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Left, VPosition::Middle).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Left, VPosition::Bottom).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Center, VPosition::Top).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Center, VPosition::Middle).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Center, VPosition::Bottom).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Right, VPosition::Top).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Right, VPosition::Middle).iter().cloned());
    stack.extend(&mut empty_stack(&mut board, HPosition::Right, VPosition::Bottom).iter().cloned());
    return stack;
}

fn empty_stack(board: &mut Board, x: HPosition, y: VPosition) -> Vec<Card> {
    let mut stack = Vec::<Card>::new();
    loop {
        let result = board.pop(&vec![Position { x: x, y: y }]);
        if result.is_none() {
            break;
        } else {
            stack.push(result.unwrap()[0])
        }
    }
    return stack;
}

fn check_count(x: HPosition, y: VPosition, count: usize) {
    let mut board = Board::new();
    assert_eq!(board.count_cards(Position { x: x, y: y }), count);
    for _ in (0..count) {
        assert!(board.pop(&vec![Position { x: x, y: y }]).is_some());
    }
    assert!(board.pop(&vec![Position { x: x, y: y }]).is_none());
}
