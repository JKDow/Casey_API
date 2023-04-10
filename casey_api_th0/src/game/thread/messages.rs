use crate::game::players::Player;
use crate::errors::PlayerError;
use crate::game::cards::Card;

//The game request is a message players send to the game making requests for infomation 
pub(crate) enum GameMessageType {
    
}

pub(crate) struct GameMessage {
    player_num: u8,
    request: GameMessageType,
}

//Messages the game sends to the players
pub(crate) enum PlayerMessage {
    GameStarted(Vec<Card>)
}

//Message type the admin sends to the game and the reply 
pub(crate) enum AdminMessage {
    TakePlayer(u8),
    InsertPlayer(Player),
}

pub(crate) enum AdminReply {
    TakePlayer(Result<Player, PlayerError>),
    InsertPlayer(Result<(), PlayerError>),
}
