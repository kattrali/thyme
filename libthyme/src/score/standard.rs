extern crate cards;

use board::{Position,VPosition};
use game::MoveType;
use score::{Play,Score,Scorer};

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
