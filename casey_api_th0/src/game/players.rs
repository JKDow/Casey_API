/* 
Contains the player struct
This is the interface through which the player interacts with the game
Messages will be sent and received from the game
The get_msg() function should be regularly called to stay up to date with game
*/


use crate::game::cards::Card;
use std::sync::mpsc;
use crate::game::thread::messages::{GameMessage, PlayerMessage};
use crate::errors::PlayerMessageError;

#[derive(Debug)]
pub struct Player {
    pub(crate) game_id: u32,
    pub(crate) player_num: u8,
    pub(crate) team_num: u8,
    pub(crate) hand: Vec<Card>,

    tx: mpsc::Sender<GameMessage>,
    rx: mpsc::Receiver<PlayerMessage>,
}

impl Player {
    /* 
    Name: new()
    Description: Creates a new player struct
    Params: 
        game_id: u32 - the game id
        player_num: u8 - the player number
        team_num: u8 - the team number
        coms: (mpsc::Sender<GameMessage>, mpsc::Receiver<PlayerMessage>) - the communication channels to the game
    Returns: Player - the player struct
    */
    pub(crate) fn new(game_id: u32, player_num: u8, team_num: u8, coms: (mpsc::Sender<GameMessage>, mpsc::Receiver<PlayerMessage>)) -> Player {
        Player {
            game_id,
            player_num,
            team_num,
            hand: Vec::new(),

            tx: coms.0,
            rx: coms.1,
        }
    }

    /* 
    Name: set_hand()
    Description: 
        Sets the player's hand
        This is a helper function for if the hand needs to be undated outide of dealing
    Params: 
        deck: Vec<Card> - the player's hand
    Returns: none
    */
    pub(crate) fn set_hand (&mut self, deck: Vec<Card>) {
        self.hand = deck;
    }

    /* 
    Name: get_msg()
    Description: 
        Gets a message from the game
        This should be called regularly to stay up to date with the game
    Params: none
    Returns: Result<PlayerMessage, PlayerMessageError> - the message from the game or an error detailing why it failed
    */
    pub fn get_msg(&self) -> Result<PlayerMessage, PlayerMessageError> {
        match self.rx.try_recv() {
            Ok(msg) => Ok(msg),
            Err(e) => {
                Err(PlayerMessageError::from(e))
            }
        }
    }

    /* 
    Name: get_hand()
    Description: Gets a reference to the player's hand
    Params: none
    Returns: &Vec<Card> - the player's hand
    */
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    /* 
    Name: get_info()
    Description: Gets the player's info
    Params: none
    Returns: (u8, u8) - the player's number and team number
    */
    pub fn get_info(&self) -> (u8, u8) {
        (self.player_num, self.team_num)
    }
}