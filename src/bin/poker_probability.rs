mod cards;

use crate::cards::deck::Deck;
use crate::cards::hand::Hand;

fn main() {
    let mut deck: Deck = Deck::full();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
    let hand: Hand = Hand::from_deck(&deck);
    println!("{}", hand);
}