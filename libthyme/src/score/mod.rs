extern crate cards;

pub mod standard;
pub mod vegas;

use board::Position;
use game::MoveType;

pub struct Score {
    /// The increase in score attained by playing this hand
    pub value: i32,
    /// A score bonus from playing this hand, such as from clearing a stack
    pub bonus: i32,
    /// Score multiplier from playing a lucky hand
    pub multiplier: i32,
}

#[derive(Clone)]
pub struct Play {
    /// Cards played
    pub cards: Vec<cards::card::Card>,
    /// All positions/stacks cleared by this play
    pub cleared_positions: Vec<Position>,
    /// Type of play
    pub hand: MoveType,
}

pub trait Scorer {

    /// Creates a new scorer. Scores may take the lucky card into account, so
    /// it is provided as a helper.
    fn new(lucky_card: cards::card::Card) -> Self;

    /// Compute the score of a potential play
    fn check_play(&self, play: Play) -> Score;

    /// Update the score with information about the last play
    fn add_play(&mut self, play: Play);

    /// Bonus awarded for clearing a position
    fn bonus(&self, position: Position) -> i32;

    /// The score including any completion bonuses or multipliers, etc
    fn score(&self, completion: bool) -> i32;

    /// Format a value as a score
    fn format_as_score(&self, value: i32) -> String;
}

