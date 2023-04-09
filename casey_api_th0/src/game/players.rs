use crate::game::cards::Card;

pub struct Player {
    pub(crate) game_id: u32,
    pub(crate) player_num: u8,
    pub(crate) team_num: u8,
    pub(crate) hand: Vec<Card>,
}

impl Player {
    pub(crate) fn new(game_id: u32, player_num: u8, team_num: u8) -> Player {
        Player {
            game_id,
            player_num,
            team_num,
            hand: Vec::new(),
        }
    }
}