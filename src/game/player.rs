use std::collections::HashMap;
use crate::errors::{TurnError, TempMeldError};
use super::cards::*;

#[derive(Debug, PartialEq)]
pub(crate) enum TurnPhase {
    Not,
    Draw,
    Throw,
}

#[derive(Debug)]
pub struct Player {
    pub(crate) hand: Vec<Card>,
    pub(crate) turn_phase: TurnPhase,

    id: usize,
    melds: HashMap<usize, Meld>,
    temp: Option<Meld>,
}

impl Player {
    pub(crate) fn new(id: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: HashMap::new(), 
            temp: None,
            turn_phase: TurnPhase::Not,
        }
    }

    //returns true if its the players turn
    pub fn my_turn(&self) -> bool {
        self.turn_phase != TurnPhase::Not
    }

    //returns a reference to the players hand
    pub fn get_hand(&self) -> &Vec<Card> { //returns reference to hand of player
        return &self.hand;
    }

    //Attemp to throw a card from the players hand
    //returns result containing the card or error
    pub fn throw(&mut self, card: usize) -> Result<Card, TurnError>{ //throws card with 'card' index
        if self.turn_phase != TurnPhase::Throw {
            return Err(TurnError::not_throw_phase("Can only throw on your turn and after drawing a card")); //Display error phase 
        }
        if card >= self.hand.len() {
            return Err(TurnError::invalid_card("Card index is out of bounds"));
        }
        self.turn_phase = TurnPhase::Not;
        return Ok(self.hand.remove(card));
    }

    //returns a referene to the card or error containing the passed card
    pub fn draw_deck(&mut self, card: Card) -> Result<&Card, TurnError>{ //draw top card from the deck
        if self.turn_phase != TurnPhase::Draw {
            return Err(TurnError::not_draw_phase("Can only draw on your turn when you havnt already", card)); //Display error phase 
        }
        self.hand.push(card);
        return Ok(&self.hand[self.hand.len()-1]);
    }

    // function that pushes a card from current players hand to temp melds 
    // returns result
    pub fn push_temp(&mut self, card: usize) -> Result<&Vec<Card>, TempMeldError> {
        // if index is out of range return error
        if card >= self.hand.len() {
            return Err(TempMeldError::card_number("Card number out of range"));
        }
        //take card from deck
        let card = self.hand.remove(card);
        //check if theres already a meld 
        match self.temp.take() {
            None => { //if not create a new one
                match Meld::new(card) {
                    Ok(meld) => {
                        self.temp = Some(meld);
                        if let Some(ref x) = self.temp {
                            return Ok(x.get_cards());
                        } else {return Err(TempMeldError::card_number("How did you reach this error??"))}
                    }
                    Err(e) => {
                        let card = e.get_card().unwrap();
                        self.hand.push(card);
                        return Err(TempMeldError::from(e));
                    }
                }
            }
            Some(mut meld) => {
                match meld.add(card) {
                    Ok(_) => {
                        self.temp = Some(meld);
                        if let Some(ref x) = self.temp {
                            return Ok(x.get_cards());
                        } else {return Err(TempMeldError::card_number("How did you reach this error??"))}
                    }
                    Err(e) => {
                        self.temp = Some(meld);
                        let card = e.get_card().unwrap();
                        self.hand.push(card);
                        return Err(TempMeldError::from(e));
                    }
                }
            }
        }
    }

    //returns all cards from temp to hand 
    //sets temp to none
    pub fn clear_temp(&mut self) {
        if let Some(meld) = self.temp.take() {
            for card in meld.clear() {
                self.hand.push(card);
            }
        }
    }    

    //returns a reference to the temp meld
    pub fn get_temp(&self) -> Option<&Vec<Card>> {
        if let Some(ref meld) = self.temp {
            return Some(meld.get_cards());
        }
        return None;
    }

    //takes temp and moves it to melds if it is valid 
    //reference to melds if ok or temp meld error if not
    pub fn commit_temp(&mut self) -> Result<&Vec<Meld>, TempMeldError> {
        if let Some(meld) = self.temp.take() {
            match meld.commit() {
                Ok(_) => {
                    self.melds.push(meld);
                    return Ok(&self.melds);
                }
                Err(e) => {
                    self.temp = Some(meld);
                    return Err(TempMeldError::from(e));
                }
            }
        }
        return Err(TempMeldError::no_meld("No meld to commit"));
    }
}