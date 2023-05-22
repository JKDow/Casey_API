/* 
This file handles all the logic for melds
All the validation needs to be updated
Its fairly not good right now
*/

use crate::card::{Suit, Card, Rank};

pub struct Meld {
    suit: Suit,
    cards: Vec<Card>,
    natural: bool,
    value: usize 
}

impl Meld {
    pub(crate) fn new(suit: Suit, mut cards: Vec<Card>) -> Result<Meld, Vec<Card>> {
        let mut meld = Meld {
            suit: suit,
            cards: Vec::new(),
            natural: true,
            value: 0,
        };
        meld.add_cards(cards)?;
        Ok(meld)
    }

    pub(crate) fn add_cards(&mut self, mut cards: Vec<Card>) -> Result<(), Vec<Card>> {
        // This is horrible and I need to fix it 
        let backup = cards.clone();
        let backup2 = self.cards.clone();
        while cards.len() != 0 {
            let card = cards.pop().unwrap();
            if let Err(_) = self.add_card(card) {
                self.cards = backup2;
                return Err(backup);
            }
        }
        Ok(())
    }

    fn add_card(&mut self, card: Card) -> Result<(), Card> {
        if card.rank == Rank::Three {
            return Err(card);
        }
        // handle if card is a wild
        if card.is_wild() {
            // if canasta is a natural then adding a wild will update that 
            if self.natural {
                self.natural = false;
                self.cards.push(card);
                self.update_value();
                return Ok(())
            }
            // check if adding card would increase number of wilds above normal cards 
            let mut counter = 0;
            for card in &self.cards {
                if card.is_wild() {
                    counter += 1;
                }
            }
            if counter >= self.cards.len() - counter {
                return Err(card); // too many wilds to add another 
            }
            self.cards.push(card);
            self.update_value();
            return Ok(())
        }
        if card.suit != self.suit {
            return Err(card);
        }
        self.cards.push(card);
        self.update_value();
        return Ok(());
    }

    fn update_value(&mut self) {
        let mut value: usize = 0;
        for card in &self.cards {
            value += card.value as usize;
        }
        if self.cards.len() >= 7 {
            if self.natural {
                value += 500;
            } else {
                value += 300;
            }
        }
        self.value = value;
    }
}