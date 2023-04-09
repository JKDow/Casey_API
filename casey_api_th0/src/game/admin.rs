/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use crate::game::players::Player;
use crate::setup::GameSettings;
use crate::game::cards::*;
//contains players, deck, discard, melds, msg channels, tread handles
pub struct GameAdmin {
    running: bool,
    players: Vec<Option<Player>>, //vec of players - is option to allow for empty slots
    deck: Vec<Card>,    
    discard: Vec<Card>,
    melds: Vec<Vec<Meld>>, //vec of vecs of melds - one vec of melds for each team 
    //msg in
    //msg out vec
    //thread handles
}

impl GameAdmin {
    pub(crate) fn new (settings: &GameSettings) -> GameAdmin {
        GameAdmin {
            running: false,
            players: Vec::new(),
            deck: make_deck(),
            discard: Vec::new(),
            melds: Vec::new(),
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

//helper function that creates the deck 
fn make_deck() -> Vec<Card> { 
    use rand::prelude::*;
    let mut deck = Vec::new();
    for i in 0..=13 {
        deck.push(Card::new(Suit::Heart, i));
    }
    for i in 0..=13 {
        deck.push(Card::new(Suit::Diamond, i));
    }
    for i in 0..=13 {
        deck.push(Card::new(Suit::Spade, i));
    }
    for i in 0..=13 {
        deck.push(Card::new(Suit::Club, i));
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    return deck;
}