extern crate cards;

#[derive(PartialEq, Clone, Copy)]
pub enum VPosition {
    Top,
    Middle,
    Bottom,
}

#[derive(PartialEq, Clone, Copy)]
pub enum HPosition {
    Left,
    Center,
    Right,
}

pub enum BoardError {
    NoCardInStack
}

#[derive(PartialEq, Clone, Copy)]
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

    /// View the cards on top of the stack at a selection of positions, or a
    /// BoardError if the request could not be fulfilled for all positions
    pub fn peek(&mut self, positions: &Vec<Position>) -> Result<Vec<cards::card::Card>, BoardError> {
        return self.take_last(positions, false)
    }

    /// View the top card of any stack
    pub fn top(&mut self, position: Position) -> Option<cards::card::Card> {
        let result = self.take_last(&vec![position], false);
        if result.is_ok() {
            return Some(result.ok().unwrap()[0])
        }
        return None;
    }

    /// View and remove the cards on the of the stacks at a selection of
    /// positions, or a BoardError if the request could not be fulfilled for
    /// all positions
    pub fn pop(&mut self, positions: &Vec<Position>) -> Result<Vec<cards::card::Card>, BoardError> {
        return self.take_last(positions, true)
    }

    fn take_last(&mut self, positions: &Vec<Position>, remove_matches: bool) -> Result<Vec<cards::card::Card>, BoardError> {
        let mut selection = Vec::new();
        for index in 0..self.stacks.len() {
            let ref mut stack = self.stacks[index];
            if positions.contains(&stack.position) {
                if stack.cards.len() > 0 {
                    let card = if remove_matches {
                        stack.cards.pop().unwrap()
                    } else {
                        stack.cards[stack.cards.len() - 1]
                    };
                    selection.push(card)
                } else {
                    return Err(BoardError::NoCardInStack)
                }
            }
        }
        return Ok(selection)
    }
}

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
