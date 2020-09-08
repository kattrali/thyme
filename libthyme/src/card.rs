use cards::card::{Card,Value};

/// Check that a selection of cards contains cards with the same value a given
/// number of times
pub fn contains_multiple_of_value(cards: &Vec<Card>, times: usize) -> bool {
    for card in cards {
        if cards.iter().filter(|c| c.value == card.value).count() == times {
            return true
        }
    }
    return false
}

/// Check that a selection of cards has consecutive values
pub fn is_consecutive(cards: &mut Vec<Card>) -> bool {
    if cards.len() < 2 {
        return false
    }
    let ace_low = cards.iter().filter(|a| a.value == Value::Two).count() > 0;
    cards.sort_by(|a, b| sort_value(a.value, ace_low).cmp(&sort_value(b.value, ace_low)));
    let mut values = cards.iter().map(|&a| sort_value(a.value, ace_low)).peekable();
    for _ in 0..values.len() - 1 {
        if values.next().unwrap() - values.peek().unwrap() != -1 {
            return false
        }
    }
    return true
}

/// Check that a selection of cards share a single suit
pub fn is_same_suit(cards: &Vec<Card>) -> bool {
    return cards.len() > 1 && cards.iter().all(|&a| a.suit == cards[0].suit)
}

fn sort_value(value: cards::card::Value, ace_low: bool) -> i32 {
    return match value {
        Value::Two => 0,
        Value::Three => 1,
        Value::Four => 2,
        Value::Five => 3,
        Value::Six => 4,
        Value::Seven => 5,
        Value::Eight => 6,
        Value::Nine => 7,
        Value::Ten => 8,
        Value::Jack => 9,
        Value::Queen => 10,
        Value::King => 11,
        Value::Ace => if ace_low { -1 } else { 12 },
    }
}
