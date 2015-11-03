extern crate cards;

use board::{Position,HPosition,VPosition};
use game::MoveType;

pub struct Score {
    /// The increase in score attained by playing this hand
    pub value: i32,
    /// A score bonus from playing this hand, such as from clearing a stack
    pub bonus: i32,
    /// Score multiplier from playing a lucky hand
    pub multiplier: i32,
}

pub trait Scorer {

    /// Creates a new scorer. Scores may take the lucky card into account, so
    /// it is provided as a helper.
    fn new(lucky_card: cards::card::Card) -> Self;

    /// Compute the score of a potential play
    fn check_play(&self, hand: MoveType, emptied_positions: Vec<Position>) -> Score;

    /// Update the score with information about the last play
    fn add_play(&mut self, hand: MoveType, emptied_positions: Vec<Position>);

    /// The in-game total score
    fn running_total(&self) -> String;

    /// The final score including any completion bonuses or multipliers, etc
    fn final_total(&self, completion: bool) -> String;
}

pub struct StandardScorer {
    lucky_suit: cards::card::Suit,
    total: i32,
}

impl Scorer for StandardScorer {

    fn new(lucky_card: cards::card::Card) -> StandardScorer {
        StandardScorer {
            total: 0,
            lucky_suit: lucky_card.suit,
        }
    }

    fn check_play(&self, hand: MoveType, emptied_positions: Vec<Position>) -> Score {
        Score {
            value: 0,
            bonus: 0,
            multiplier: 0,
        }
    }

    fn add_play(&mut self, hand: MoveType, emptied_positions: Vec<Position>) {
    }

    fn running_total(&self) -> String {
        return format!("{}", self.total)
    }

    fn final_total(&self, completion: bool) -> String {
        return format!("{}", self.total)
    }
}

