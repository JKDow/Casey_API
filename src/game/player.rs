use crate::errors::TurnError;
use super::cards::*;
use super::*;

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
    melds: Vec<Meld>,
    temp: Vec<Meld>,
}

impl Player {
    pub(crate) fn new(id: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: vec![], 
            temp: vec![],
            turn_phase: TurnPhase::Not,
        }
    }

    pub fn my_turn(&self) -> bool {
        self.turn_phase != TurnPhase::Not
    }

    pub fn get_hand(&self) -> &Vec<Card> { //returns reference to hand of player
        return &self.hand;
    }

    pub fn throw(&mut self, card: usize) -> Result<Card, TurnError>{ //throws card with 'card' index
        if self.turn_phase != TurnPhase::Throw {
            return Err(TurnError::turn_phase("Can only throw on your turn and after drawing a card")); //Display error phase 
        }
        if card >= self.hand.len() {
            return Err(TurnError::invalid_card("Card index is out of bounds"));
        }
        self.turn_phase = TurnPhase::Not;
        return Ok(self.hand.remove(card));
    }

    //check its your turn with error type 
    pub fn draw_deck(&mut self, game: &mut Game) { //draw top card from the deck
        let card = game.deck.pop();
        match card {
            Some(card) => {
                self.hand.push(card);
            }
            None => {
                //End game
            }
        }
    }

    // function that pushes a card from current players hand to temp melds 
    //returns bool indicating if it was successful 
    pub fn push_temp(&mut self, card: usize) -> bool {
        todo!();
    }

    pub fn push_temp_wild(&mut self, card: usize, rank: usize) -> bool {
        todo!();
    }

    pub fn clear_temp(&mut self) -> bool {
        todo!();
    }    

    pub fn clear_temp_rank(&mut self) -> bool {
        todo!();
    }

}