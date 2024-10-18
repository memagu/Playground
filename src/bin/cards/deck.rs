use std::fmt;

use crate::cards::card::{Card, Rank, Suit, RANKS, SUITS};
use crate::cards::xorshift::XorShift;

pub const DECK_SIZE: usize = 52;

#[derive(Debug, Copy, Clone)]
pub struct Deck {
    pub cards: [Card; DECK_SIZE],
    rng: XorShift,
}

impl Deck {
    pub fn new(cards: [Card; DECK_SIZE]) -> Self {
        Self { cards, rng: XorShift::new() }
    }

    pub fn full() -> Self {
        let ranks: [Rank; RANKS] = Rank::all();
        let suits: [Suit; SUITS] = Suit::all();

        let cards: [Card; DECK_SIZE] = std::array::from_fn(|i: usize| Card::new(ranks[i / SUITS], suits[i % SUITS]));
        Self::new(cards)
    }

    pub fn shuffle(&mut self) -> () {
        for i in (0..self.cards.len()).rev() {
            let random_index: usize = self.rng.gen_range(0, i + 1);
            self.cards.swap(i, random_index);
        }
    }

    pub fn peek(&self, n: usize) -> Result<&[Card], &str> {
        if n >= DECK_SIZE {
            return Err("n must lie in [0, {51}]");
        }
        Ok(&self.cards[..n])
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_strings: String = self.cards.iter()
            .map(|card: &Card| format!("{}", card))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", card_strings)
    }
}
