extern crate cards;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VPosition {
    Top,
    Middle,
    Bottom,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HPosition {
    Left,
    Center,
    Right,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    /// horizontal position on the board
    pub x: HPosition,
    /// vertical position on the board
    pub y: VPosition,
}

pub struct Stack {
    /// location of the stack on the board
    pub position: Position,
    /// cards in the stack, ordered bottom to top
    pub cards: Vec<cards::card::Card>
}

pub struct Board {
    /// stacks of cards in play, by position
    pub stacks: Vec<Stack>,
    /// the 'lucky' card, not present in any stack
    pub lucky_card: cards::card::Card,
}

impl Board {
    /// Create a new board, dealing a 52-card deck into 9 stacks and one
    /// lucky 'extra' card
    pub fn new() -> Board {
        let (stacks, lucky_card) = deal_deck();
        Board { stacks: stacks, lucky_card: lucky_card }
    }

    /// Positions on the board
    pub fn positions(&self) -> Vec<Position> {
        return self.stacks.iter().map(|s| s.position).collect();
    }

    /// Any positions with cards remaining on the board
    pub fn positions_remaining(&self) -> Vec<Position> {
        return self.stacks.iter().filter(|s| s.cards.len() > 0).map(|s| s.position).collect()
    }

    /// All possible combinations of the remaining positions with cards on the
    /// board
    pub fn hands_remaining(&self) -> Vec<Vec<Position>> {
        return combination(&self.positions_remaining())
    }

    /// Count the remaining cards in a stack
    pub fn count_cards(&self, position: Position) -> usize {
        let stack = self.stacks.iter().filter(|s| s.position == position).last();
        if stack.is_some() {
            return stack.unwrap().cards.len()
        }
        return 0;
    }

    /// Count the remaining cards on the board
    pub fn count_all_cards(&self) -> usize {
        return self.stacks.iter().fold(0, |acc, s| acc + s.cards.len());
    }

    /// View the cards on top of the stack at a selection of positions, or None
    /// if the request could not be fulfilled for all positions
    pub fn peek(&mut self, positions: &Vec<Position>) -> Option<Vec<cards::card::Card>> {
        return self.take_last(positions, false)
    }

    /// View the top card of any stack
    pub fn top(&mut self, position: Position) -> Option<cards::card::Card> {
        let result = self.take_last(&vec![position], false);
        return if result.is_some() { Some(result.unwrap()[0]) } else { None }
    }

    /// View and remove the cards on the of the stacks at a selection of
    /// positions, or None if the request could not be fulfilled for
    /// all positions
    pub fn pop(&mut self, positions: &Vec<Position>) -> Option<Vec<cards::card::Card>> {
        return self.take_last(positions, true)
    }

    fn take_last(&mut self, positions: &Vec<Position>, remove_matches: bool) -> Option<Vec<cards::card::Card>> {
        let remaining = self.positions_remaining();
        for position in positions {
            if !remaining.contains(position) {
                return None
            }
        }
        let stacks = self.stacks.iter_mut().filter(|s| positions.contains(&s.position));
        return Some(stacks.map(|s| if remove_matches {
            s.cards.pop().unwrap()
        } else {
            *s.cards.last().unwrap()
        }).collect())
    }
}

/// Divide deck into stacks
fn deal_deck() -> (Vec<Stack>, cards::card::Card) {
    let mut deck = cards::deck::Deck::new_shuffled();
    let stacks = vec![
        Stack { position: Position { x: HPosition::Left, y: VPosition::Top },
                cards: deck.draw_n(8).ok().unwrap() },
        Stack { position: Position { x: HPosition::Center, y: VPosition::Top },
                cards: deck.draw_n(8).ok().unwrap() },
        Stack { position: Position { x: HPosition::Right, y: VPosition::Top },
                cards: deck.draw_n(8).ok().unwrap() },
        Stack { position: Position { x: HPosition::Left, y: VPosition::Middle },
                cards: deck.draw_n(7).ok().unwrap() },
        Stack { position: Position { x: HPosition::Center, y: VPosition::Middle },
                cards: deck.draw_n(6).ok().unwrap() },
        Stack { position: Position { x: HPosition::Right, y: VPosition::Middle },
                cards: deck.draw_n(5).ok().unwrap() },
        Stack { position: Position { x: HPosition::Left, y: VPosition::Bottom },
                cards: deck.draw_n(4).ok().unwrap() },
        Stack { position: Position { x: HPosition::Center, y: VPosition::Bottom },
                cards: deck.draw_n(3).ok().unwrap() },
        Stack { position: Position { x: HPosition::Right, y: VPosition::Bottom },
            cards: deck.draw_n(2).ok().unwrap() }];
    return (stacks, deck.draw().ok().unwrap())
}

/// Find all k-combinations of a set of positions
fn combination(positions: &Vec<Position>) -> Vec<Vec<Position>> {
    return combine(&mut vec![], positions)
}

fn combine(prefix: &mut Vec<Position>, positions: &Vec<Position>) -> Vec<Vec<Position>> {
    let result = (0..positions.len()).map(|i|{
        let mut items = vec![positions[i]];
        items.extend(prefix.to_vec());
        let mut pos = vec![];
        for j in i+1..positions.len() {
            pos.push(positions[j])
        }
        return combine(&mut items, &pos)
    });
    let mut accumulator = vec![prefix.to_vec()];
    for item in result {
        accumulator.extend(item);
    }
    return accumulator
}
