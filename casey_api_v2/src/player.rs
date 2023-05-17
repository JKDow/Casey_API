use std::collections::HashMap;
use crate::meld::Meld;
use crate::card::{Suit, Card};

pub struct Player{
    id: usize,
    pub(crate) hand: Vec<Card>,
    pub(crate) melds: HashMap<Suit, Meld>,
}

impl Player {
    pub(crate) fn new(id: usize) -> Player {
        Player {
            id,
            hand: Vec::new(),
            melds: HashMap::new(),
        }
    }

    
}