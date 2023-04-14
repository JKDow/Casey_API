use crate::game::players::Player;
use crate::errors::PlayerError;
use crate::game::cards::Card;

use super::core::Game;

/* 
Name: Game Message
Description: Message sent from a player to the game to request information and respond with actions
*/
pub(crate) enum GameMessageType {
    
}

pub(crate) struct GameMessage {
    player_num: u8,
    request: GameMessageType,
}

/* 
Name: Player Message
Description: Message sent from the game to a player with updates and requests
Types:
    GameStarted - the game has started - gives a copy of the current discard pile, 0 is the lowwest card
    ActionRequest - the player needs to respond with an action
*/
pub enum PlayerMessageType {
    GameStarted(Vec<Card>),
    ActionRequest(ActionRequest),
    DenyAction(ActionDeny),
    GameUpdate,
}

pub struct PlayerMessage {
    msg_type: Option<PlayerMessageType>,
    text: String,
}

impl PlayerMessage {
    pub(crate) fn new(msg_type: PlayerMessageType, text: &str) -> PlayerMessage {
        PlayerMessage {
            msg_type: Some(msg_type),
            text: String::from(text),
        }
    }

    pub fn get_type(&mut self) -> Option<PlayerMessageType> {
        self.msg_type.take()
    }

    pub fn read(&self) -> String {
        self.text.clone()
    }
}

/* 
Name: Admin Message and Admin Reply
Description: Message pair for the admin with a set of commands 
*/
pub(crate) enum AdminMessage {
    TakePlayer(u8),
    InsertPlayer(Player),
    PauseGame,
    ResumeGame,
    EndGame,
}

pub(crate) enum AdminReply {
    TakePlayer(Result<Player, PlayerError>),
    InsertPlayer(Result<(), PlayerError>),
    PauseGame(Result<(), ()>),
    ResumeGame(Result<(), ()>),
    EndGame(Result<Game, ()>),
}

/* 
Name: Action Object
Description: 
    The object is sent from the game to a player with a request
    The player then runs commands on that object and sends it back to the game
    The object will contain the action type it wants and a unique code to ensure reuse of action objects does not occur 
*/

//Request
pub enum ActionRequestType {
    Draw,
    MeldThrow,
}

pub struct ActionRequest {
    pub(crate) code: u32,  //code for the action to ensure that the correct reply is being used 
    pub(crate) action_type: ActionRequestType, //the type of action the player is being asked to do
}

//Reply
pub(crate) enum ActionReplyType {
    Draw,
    TakeDiscard(Vec<Card>),
    Meld(Vec<Card>),
    Throw(Card),
}

pub(crate) struct ActionReply {
    code: u32,  //code for the action to ensure that the correct reply is being used 
    action_type: ActionReplyType, //the type of action the player is being asked to do
    hand_backup: Vec<Card>, //the hand of the player before the action was taken
}

//deny action
pub enum ActionDenyType {
    InvalidCardsTakePack,
    InvalidCardsMeld,
    FirstMeldPoints
}

pub struct ActionDeny {
    deny_type: ActionDenyType,
    msg: String,
}