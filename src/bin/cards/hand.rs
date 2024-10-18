use std::fmt;

use crate::cards::card::Card;
use crate::cards::deck::Deck;

pub const HAND_SIZE: usize = 5;

#[derive(Debug, Copy, Clone)]
pub struct Hand {
    cards: [Card; HAND_SIZE],
}

impl Hand {
    pub fn new(cards: [Card; HAND_SIZE]) -> Self {
        Self { cards }
    }

    pub fn from_deck(deck: &Deck) -> Self {
        Self::new(deck.peek(HAND_SIZE).unwrap().try_into().unwrap())
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_strings: String = self.cards.iter()
            .map(|card: &Card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", card_strings)
    }
}