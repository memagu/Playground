use std::fmt;

use crate::cards::card::{ Card, RANKS, Rank };
use crate::cards::deck::Deck;

pub const HAND_SIZE: usize = 5;
pub const CATEGORIES: usize = 10;

#[derive(Debug, Copy, Clone)]
pub enum Category {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Category {
    pub fn all() -> [Category; CATEGORIES] {
        [
            Category::RoyalFlush,
            Category::StraightFlush,
            Category::FourOfAKind,
            Category::FullHouse,
            Category::Flush,
            Category::Straight,
            Category::ThreeOfAKind,
            Category::TwoPairs,
            Category::OnePair,
            Category::HighCard,
        ]
    }
    
    pub fn ordinal(&self) -> usize {
        match self {
            Self::RoyalFlush => 0,
            Self::StraightFlush => 1,
            Self::FourOfAKind => 2,
            Self::FullHouse => 3,
            Self::Flush => 4,
            Self::Straight => 5,
            Self::ThreeOfAKind => 6,
            Self::TwoPairs => 7,
            Self::OnePair => 8,
            Self::HighCard => 9,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::RoyalFlush => "royal flush",
            Self::StraightFlush => "straight flush",
            Self::FourOfAKind => "four of a kind",
            Self::FullHouse => "full house",
            Self::Flush => "flush",
            Self::Straight => "straight",
            Self::ThreeOfAKind => "three of a kind",
            Self::TwoPairs => "two pairs",
            Self::OnePair => "one pair",
            Self::HighCard => "high card",
        })
    }
}

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

    fn talley(&self) -> [usize; RANKS] {
        let mut talley: [usize; RANKS] = [0; RANKS];
        for card in self.cards {
            talley[card.rank.ordinal()] += 1;
        }
        talley
    }

    fn is_straigt(&self) -> bool {
        let mut ranks: [Rank; HAND_SIZE] = std::array::from_fn(|i| self.cards[i].rank);
        ranks.sort_unstable();

        let is_non_royal_straight: bool = ranks.as_slice()
            .windows(2)
            .map(|window: &[Rank]| window[1].ordinal() - window[0].ordinal())
            .all(|difference: usize| difference == 1);

        is_non_royal_straight || ranks == [Rank::Ace, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King]
    }
 
    pub fn category(&self) -> Category {
        let talley: [usize; RANKS] = self.talley();
        let is_flush: bool = self.cards.iter().all(|&card| card.suit == self.cards[0].suit);
        
        if talley.contains(&2) {
            if talley.contains(&3) { return Category::FullHouse } 
            if talley.iter().filter(|&&count| count == 2).count() == 2 { return Category::TwoPairs }
            return Category::OnePair
        }
        if talley.contains(&4) { return Category::FourOfAKind }
        if talley.contains(&3) { return Category::ThreeOfAKind }
        if self.is_straigt() {
            if is_flush {
                if talley[0] == 1 && talley[12] == 1 { return Category::RoyalFlush }
                return Category::StraightFlush
            } 
            return Category::Straight
        }
        if is_flush { return Category::Flush }
        Category::HighCard
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
