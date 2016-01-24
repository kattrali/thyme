extern crate cards;

use board::{Position,VPosition};
use game::MoveType;
use score::{Play,Score,Scorer};

/// Vegas scoring hands, money only awarded for hands including a card with the
/// lucky card's suit
pub struct VegasScorer {
    lucky_suit: cards::card::Suit,
    total: i32,
    multiplier: f32,
}

impl Scorer for VegasScorer {

    fn new(lucky_card: cards::card::Card) -> VegasScorer {
        VegasScorer {
            total: 0,
            lucky_suit: lucky_card.suit,
            multiplier: 0.0,
        }
    }

    fn check_play(&self, play: Play) -> Score {
        let lucky_card = play.cards.iter().find(|c| c.suit == self.lucky_suit);
        Score {
            value: if lucky_card.is_some() { move_value(play.hand) } else { 0 },
            bonus: 0,
            multiplier: 1,
        }
    }

    fn bonus(&self, _: Position) -> i32 {
        return 0;
    }

    fn add_play(&mut self, play: Play) {
        let lucky_card = play.cards.iter().find(|c| c.suit == self.lucky_suit);
        let positions = play.cleared_positions;
        for position in positions {
            self.multiplier += position_bonus_multiplier(position);
        }
        if lucky_card.is_some() {
            self.total += move_value(play.hand);
        }
    }

    fn score(&self, completed: bool) -> i32 {
        if completed {
            return (self.total as f32 * self.multiplier) as i32
        } else {
            return self.total
        }
    }

    fn format_as_score(&self, value: i32) -> String {
        return format!("${} x {}", value, self.multiplier)
    }
}

fn position_bonus_multiplier(position: Position) -> f32 {
    return match position.y {
        VPosition::Top => 0.75,
        VPosition::Middle => 0.25,
        VPosition::Bottom => 0.0,
    }
}

fn move_value(hand: MoveType) -> i32 {
    return match hand {
        MoveType::StraightFlush => 40,
        MoveType::FourOfAKind => 20,
        MoveType::Flush => 10,
        MoveType::FullHouse => 8,
        MoveType::FiveCardStraight => 5,
        MoveType::ThreeOfAKind => 3,
        MoveType::ThreeCardStraight => 2,
        MoveType::Pair => 1,
        MoveType::Trash => 0,
    }
}
