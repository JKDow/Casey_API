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
*/
pub(crate) enum PlayerMessage {
    GameStarted(Vec<Card>)
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

pub enum ActionDeny {
    InvalidCardsTakePack,
    InvalidCardsMeld,
    FirstMeldPoints
}

pub struct ActionRequest {
    code: u32,  //code for the action to ensure that the correct reply is being used 
    action_type: ActionRequestType, //the type of action the player is being asked to do
    error: Option<ActionDeny>,  //if the player responds and it is not valid then this will contain the reason 
}

//Reply
pub(crate) enum ActionReplyType {
    Draw,
    TakeDiscard(Vec<Card>),
    Meld(Vec<Card>),
    Throw(Card),
}