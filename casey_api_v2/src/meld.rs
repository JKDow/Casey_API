use crate::card::{Suit, Card};

pub struct Meld {
    suit: Suit,
    cards: Vec<Card>,
    natural: bool, 
}