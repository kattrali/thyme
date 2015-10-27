extern crate cards;

use board::{Board,Position};
use card::*;

pub struct Game {
    pub board: Board,
    /// Number of times discarding a single card is allowed
    pub discards_allowed: i32,
    /// Maximum number of times a single card can be discarded in sequence
    pub discards_allowed_max: i32,
    pub score: i32,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MoveError {
    /// The move cannot be played given the current board
    InvalidMove,
    /// The move cannot be played because it uses only a single row
    NeedMultipleRows,
    /// The board is empty
    NoMovesRemain,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MoveType {
    /// Five consecutive cards of the same suit
    StraightFlush,
    /// Four cards with the same value
    FourOfAKind,
    /// Five cards of the same suit
    Flush,
    /// Three cards with the same value and two cards with the same value
    FullHouse,
    /// Five consecutive cards of various suits
    FiveCardStraight,
    /// Three cards with the same value
    ThreeOfAKind,
    /// The consecutive cards of various or the same suit
    ThreeCardStraight,
    /// Two cards with the same value
    Pair,
    /// Any single card, removed from play
    Trash,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Move {
    pub hand: MoveType,
    /// The increase in score attained by playing this hand
    pub score: i32,
    /// A score bonus from playing this hand, such as from clearing a stack
    pub bonus: i32,
}

impl Game {

    pub fn new() -> Game {
        Game {
            board: Board::new(),
            discards_allowed: 2,
            discards_allowed_max: 2,
            score: 0
        }
    }

    /// True if any more moves can be played
    pub fn moves_remaining(&mut self) -> bool {
        for combination in self.board.hands_remaining() {
            if self.check(&combination).is_ok() {
                return true
            }
        }
        return self.discards_allowed > 0
    }

    /// Play the cards at the top of a set of stacks, updating score and
    /// discards_allowed if applicable
    pub fn play(&mut self, hand: MoveType, positions: &Vec<Position>) -> Result<Move, MoveError> {
        let check = self.check(positions);
        if check.is_ok() && check.ok().unwrap().hand == hand {
            let _ = self.board.pop(&positions);
        }
        return check;
    }

    /// Determine what move would result from playing the cards on top of a
    /// set of stacks
    pub fn check(&mut self, positions: &Vec<Position>) -> Result<Move, MoveError> {
        let rows = uniq(positions.iter().map(|p| p.y).collect());
        let result = self.board.peek(positions);
        if rows.len() < 2 && positions.len() > 1 {
            return Err(MoveError::NeedMultipleRows);
        } else if result.is_ok() {
            return self.check_cards(&mut result.ok().unwrap())
        }
        return Err(MoveError::InvalidMove)
    }

    fn check_cards(&self, cards: &mut Vec<cards::card::Card>) -> Result<Move, MoveError> {
        match cards.len() {
            5 => return check_five(cards),
            4 => return check_four(cards),
            3 => return check_three(cards),
            2 => return check_two(cards),
            1 => return check_one(self.discards_allowed),
            _ => return Err(MoveError::InvalidMove),
        }
    }
}

fn uniq<T>(items: Vec<T>) -> Vec<T> where T: PartialEq {
    let mut result = Vec::<T>::new();
    for item in items {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    return result;
}

fn check_five(cards: &mut Vec<cards::card::Card>) -> Result<Move, MoveError> {
    if is_consecutive(cards) && is_same_suit(cards) {
        return Ok(Move { hand: MoveType::StraightFlush, score: 0, bonus: 0 })
    } else if is_consecutive(cards) {
        return Ok(Move { hand: MoveType::FiveCardStraight, score: 0, bonus: 0 })
    } else if is_same_suit(cards) {
        return Ok(Move { hand: MoveType::Flush, score: 0, bonus: 0 })
    } else if contains_multiple_of_value(cards, 3) && contains_multiple_of_value(cards, 2) {
        return Ok(Move { hand: MoveType::FullHouse, score: 0, bonus: 0 })
    }
    return Err(MoveError::InvalidMove);
}

fn check_four(cards: &mut Vec<cards::card::Card>) -> Result<Move, MoveError> {
    if contains_multiple_of_value(cards, 4) {
        return Ok(Move { hand: MoveType::FourOfAKind, score: 0, bonus: 0 })
    }
    return Err(MoveError::InvalidMove);
}

fn check_three(cards: &mut Vec<cards::card::Card>) -> Result<Move, MoveError> {
    if is_consecutive(cards) {
        return Ok(Move { hand: MoveType::ThreeCardStraight, score: 0, bonus: 0 })
    } else if contains_multiple_of_value(cards, 3) {
        return Ok(Move { hand: MoveType::ThreeOfAKind, score: 0, bonus: 0 })
    }
    return Err(MoveError::InvalidMove);
}

fn check_two(cards: &mut Vec<cards::card::Card>) -> Result<Move, MoveError> {
    if contains_multiple_of_value(cards, 2) {
        return Ok(Move { hand: MoveType::Pair, score: 0, bonus: 0 })
    }
    return Err(MoveError::InvalidMove);
}

fn check_one(trashes_remaining: i32) -> Result<Move, MoveError> {
    if trashes_remaining > 0 {
        return Ok(Move { hand: MoveType::Trash, score: 0, bonus: 0 })
    }
    return Err(MoveError::InvalidMove);
}
