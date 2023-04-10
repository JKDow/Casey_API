//The admin request is a message players send to the admin making requests for infomation 
pub(crate) enum GameRequestType {
    
}

pub(crate) struct GameRequest {
    player_num: u8,
    request: GameRequestType,
}


pub(crate) enum PlayerMessageType {

}

pub(crate) struct PlayerMessage {
    msg_type: PlayerMessageType,
}