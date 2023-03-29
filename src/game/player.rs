use super::cards::*;

#[derive(Debug)]
pub struct Player {
    pub(crate) hand: Vec<Card>,

    id: usize,
    melds: Vec<Meld>,
    temp: Vec<Meld>,
    my_turn: bool,
}

impl Player {
    pub(crate) fn new(id: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: vec![], 
            temp: vec![],
            my_turn: false,
        }
    }

    pub fn draw_deck(&mut self) { //draw top card from the deck
        // let card = self.deck.pop();
        // match card {
        //     Some(card) => {
        //         self.players[self.player_turn].hand.push(card);
        //     }
        //     None => {
        //         //End game
        //     }
        // }
    }

}