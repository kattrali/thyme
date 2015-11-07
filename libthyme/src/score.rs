extern crate cards;

use board::{Position,VPosition};
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

/// Standard (no fifteens) scoring hands, double bonus awarded for cards
/// with the lucky card's suit
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

    fn check_play(&self, play: Play) -> Score {
        let lucky_card = play.cards.iter().find(|c| c.suit == self.lucky_suit);
        Score {
            value: self.move_value(play.hand),
            bonus: play.cleared_positions.iter().fold(0, |acc, p| acc + self.bonus(*p)),
            multiplier: if lucky_card.is_some() { 2 } else { 1 },
        }
    }

    fn bonus(&self, position: Position) -> i32 {
        return match position.y {
            VPosition::Top => 150,
            VPosition::Middle => 100,
            VPosition::Bottom => 50,
        }
    }

    fn add_play(&mut self, play: Play) {
        let score = self.check_play(play);
        self.total += (score.value * score.multiplier) + score.bonus;
    }

    fn score(&self, _: bool) -> i32 {
        return self.total
    }

    fn format_as_score(&self, value: i32) -> String {
        return format!("{}", value)
    }
}

impl StandardScorer {

    fn move_value(&self, hand: MoveType) -> i32 {
        return match hand {
            MoveType::StraightFlush => 150,
            MoveType::FourOfAKind => 100,
            MoveType::Flush => 90,
            MoveType::FullHouse => 70,
            MoveType::FiveCardStraight => 50,
            MoveType::ThreeOfAKind => 30,
            MoveType::ThreeCardStraight => 20,
            MoveType::Pair => 10,
            MoveType::Trash => 0,
        }
    }
}
