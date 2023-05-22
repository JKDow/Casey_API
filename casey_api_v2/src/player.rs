use std::collections::HashMap;
use crate::meld::Meld;
use crate::card::{Suit, Card};

pub struct Player{
    id: usize,
    pub(crate) hand: Vec<Card>,
    pub(crate) melds: HashMap<Suit, Meld>,
    pub(crate) red_threes: Vec<Card>,
    pub(crate) melded: bool,
}

impl Player {
    pub(crate) fn new(id: usize) -> Player {
        Player {
            id,
            hand: Vec::new(),
            melds: HashMap::new(),
            red_threes: Vec::new(),
            melded: false,
        }
    }

    pub(crate) fn sort_hand(&mut self) {
        self.hand.sort_by(|a, b| a.value.cmp(&b.value));
    }
    
    pub(crate) fn remove_card(&mut self, card_id: usize) -> Option<Card> {
        for i in 0..self.hand.len() {
            if self.hand[i].id == card_id {
                return Some(self.hand.remove(i));
            }
        }
        return None
    }

    pub(crate) fn add_cards(&mut self, cards: Vec<Card>) {
        self.hand.extend(cards);
        self.sort_hand();
    }
    
    pub(crate) fn meld(&mut self, cards: Vec<Card>, suit: Suit) -> Result<(), Vec<Card>> {
        //check if player has meld for suit 
        match self.melds.get_mut(&suit) {
            // has melds
            Some(meld) => { 
                meld.add_cards(cards)?
            }
            // does not have melds 
            None => {
                let meld = Meld::new(suit, cards)?;
                self.melds.insert(suit, meld);
            }
        }
        self.melded = true;
        Ok(())
    }
    
    pub(crate) fn check_threes(&self) -> usize {
        let mut counter = 0;
        for card in &self.hand {
            if card.is_red_three() {
                counter += 1;
            }
        }
        counter
    }

    pub(crate) fn meld_red_threes(&mut self) -> usize {
        let mut index: Vec<usize> = Vec::new();
        for (i,card) in self.hand.iter().enumerate() {
            if card.is_red_three() {
                index.push(i);
            }
        }
        for i in index.iter().rev() {
            self.red_threes.push(self.hand.remove(*i));
        }
        return index.len()
    }
}