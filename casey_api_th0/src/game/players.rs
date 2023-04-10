use crate::game::cards::Card;
use std::sync::mpsc;
use crate::game::thread::messages::{GameMessage, PlayerMessage};

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

    pub(crate) fn set_hand (&mut self, deck: Vec<Card>) {
        self.hand = deck;
    }
}