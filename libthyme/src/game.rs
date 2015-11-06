extern crate cards;

use board::{Board,Position};
use card::*;
use score::Scorer;

pub struct Game<T: Scorer> {
    pub board: Board,
    /// Number of times discarding a single card is allowed
    pub discards_allowed: i32,
    /// Maximum number of times a single card can be discarded in sequence
    pub discards_allowed_max: i32,
    /// Score calculator
    pub scorer: T,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MoveError {
    /// The move cannot be played given the current board
    InvalidMove,
    /// The move cannot be played because it uses only a single row
    NeedMultipleRows,
    /// The board is empty
    NoMovesRemain,
    /// No discards remain
    NoDiscardsRemain,
    /// Two pair is not a sage
    TwoPairIsInvalid,
    /// The move cannot be played given the current hand
    InvalidHand,
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

impl<T: Sized> Game<T> where T: Scorer {

    pub fn new() -> Game<T> {
        let board = Board::new();
        let lucky_card = cards::card::Card {
            value: board.lucky_card.value,
            suit: board.lucky_card.suit
        };
        Game {
            board: board,
            discards_allowed: 2,
            discards_allowed_max: 2,
            scorer: Scorer::new(lucky_card),
        }
    }

    /// True if any more moves can be played
    pub fn moves_remaining(&mut self) -> bool {
        return self.board.hands_remaining().iter().find(|h| self.check(&h).is_ok()).is_some()
        || self.discards_allowed > 0
    }

    /// Play the cards at the top of a set of stacks, updating score and
    /// discards_allowed if applicable
    pub fn play(&mut self, hand: MoveType, positions: &Vec<Position>) -> Result<MoveType, MoveError> {
        let check = self.check(positions);
        if check.is_ok() {
            if check.ok().unwrap() == hand {
                let _ = self.board.pop(&positions);
                if hand == MoveType::Trash {
                    self.discards_allowed -= 1;
                } else if self.discards_allowed < self.discards_allowed_max {
                    self.discards_allowed += 1;
                }
            } else {
                return Err(MoveError::InvalidHand);
            }
        }
        return check;
    }

    /// Determine what move would result from playing the cards on top of a
    /// set of stacks
    pub fn check(&mut self, positions: &Vec<Position>) -> Result<MoveType, MoveError> {
        let rows = uniq(positions.iter().map(|p| p.y).collect());
        let result = self.board.peek(positions);
        if rows.len() < 2 && positions.len() > 1 {
            return Err(MoveError::NeedMultipleRows);
        } else if result.is_some() {
            return self.check_cards(&mut result.unwrap())
        }
        return Err(MoveError::InvalidMove)
    }

    fn check_cards(&self, cards: &mut Vec<cards::card::Card>) -> Result<MoveType, MoveError> {
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

fn check_five(cards: &mut Vec<cards::card::Card>) -> Result<MoveType, MoveError> {
    if is_consecutive(cards) && is_same_suit(cards) {
        return Ok(MoveType::StraightFlush)
    } else if is_consecutive(cards) {
        return Ok(MoveType::FiveCardStraight)
    } else if is_same_suit(cards) {
        return Ok(MoveType::Flush)
    } else if contains_multiple_of_value(cards, 3) && contains_multiple_of_value(cards, 2) {
        return Ok(MoveType::FullHouse)
    }
    return Err(MoveError::InvalidMove);
}

fn check_four(cards: &mut Vec<cards::card::Card>) -> Result<MoveType, MoveError> {
    if contains_multiple_of_value(cards, 4) {
        return Ok(MoveType::FourOfAKind)
    }
    return Err(MoveError::InvalidMove);
}

fn check_three(cards: &mut Vec<cards::card::Card>) -> Result<MoveType, MoveError> {
    if is_consecutive(cards) {
        return Ok(MoveType::ThreeCardStraight)
    } else if contains_multiple_of_value(cards, 3) {
        return Ok(MoveType::ThreeOfAKind)
    }
    return Err(MoveError::InvalidMove);
}

fn check_two(cards: &mut Vec<cards::card::Card>) -> Result<MoveType, MoveError> {
    if contains_multiple_of_value(cards, 2) {
        return Ok(MoveType::Pair)
    }
    return Err(MoveError::InvalidMove);
}

fn check_one(trashes_remaining: i32) -> Result<MoveType, MoveError> {
    if trashes_remaining > 0 {
        return Ok(MoveType::Trash)
    }
    return Err(MoveError::NoDiscardsRemain);
}
