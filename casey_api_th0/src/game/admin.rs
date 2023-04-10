/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use std::sync::RwLock;
use std::thread;
use std::sync::mpsc;
use crate::errors::AdminError;
use crate::errors::PlayerError;
use crate::errors::PlayerErrorType;
use crate::game::players::Player;
use crate::setup::GameSettings;
use crate::game::cards::*;

//contains players, deck, discard, melds, msg channels, tread handles
pub struct GameAdmin {
    game_id: u32,

    running: RwLock<bool>,
    points_to_win: u16,

    pub(crate) players: Vec<Option<Player>>, //vec of players - is option to allow for empty slots
    hand_backups: Vec<Vec<Card>>, //vec of vecs of cards - one vec of cards for each player

    deck: Vec<Card>,    
    discard: Vec<Card>,
    melds: Vec<Vec<Meld>>, //vec of vecs of melds - one vec of melds for each team 
    
    rx: mpsc::Receiver<AdminRequest>, //msg in 
    player_tx: Vec<mpsc::Sender<PlayerMessage>>, //msg out to players
    handle: Option<thread::JoinHandle<()>>
}

impl GameAdmin {
    pub(crate) fn new (settings: &GameSettings) -> GameAdmin {
        let (tx_admin, rx_admin) = mpsc::channel();

        let game_id = rand::random();
        let mut players: Vec<Player> = Vec::new();
        let mut senders: Vec<mpsc::Sender<PlayerMessage>> = Vec::new();
        for i in 0..settings.num_players {
            let (tx_player, rx_player) = mpsc::channel();
            senders.push(tx_player);
            players.push(Player::new(game_id, i, i % settings.team_size, (tx_admin.clone(), rx_player)));
        }

        let mut deck = make_deck();
        for _ in 0..settings.deal_size {
            for i in 0..players.len() {
                players[i].hand.push(deck.pop().unwrap())
            }
        }

        let mut wrapped: Vec<Option<Player>> = vec![];
        for _ in 0..players.len() {
            wrapped.push(Some(players.remove(0)));
        }

        GameAdmin {
            game_id,
            running: RwLock::new(false),
            points_to_win: settings.points_to_win,
            players: wrapped,
            hand_backups: Vec::new(),
            deck,
            discard: Vec::new(),
            melds: Vec::new(),
            handle: None,

            rx: rx_admin,
            player_tx: senders,
        }
    }

    pub fn running(&self) -> bool {
        let running = self.running.read().unwrap();
        *running
    }

    pub fn take_player(&mut self, player_num: u8) -> Result<Player, PlayerError> {
        if player_num >= self.players.len() as u8 {
            return Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player number out of range"));
        }
        let player = self.players[player_num as usize].take();
        match player {
            Some(player) => Ok(player),
            None => Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player not found")),
        }
    }

    pub fn insert_player(&mut self, player: Player) -> Result<(), PlayerError> {
        //check if game id in player matches game id 
        if player.game_id != self.game_id {
            return Err(PlayerError::new(PlayerErrorType::PlayerFromWrongGame, "Player from wrong game"));
        }
        if self.players[player.player_num as usize].is_some() {
            return Err(PlayerError::new(PlayerErrorType::PlayerAlreadyInserted, "Player already exists"));
        }
        let num = player.player_num as usize;
        self.players[num] = Some(player);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), AdminError>{
        {
            let mut running = self.running.write().unwrap();
            *running = true;
        }
        let handler = thread::spawn(move || {
            loop {
                let running = self.running.read().unwrap();
                if !*running {
                    break;
                }
                //drop(running);
            }
        });

        return Ok(());
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

//The admin request is a message players send to the admin making requests for infomation 
pub(crate) enum AdminRequestType {
    
}

pub(crate) struct AdminRequest {
    player_num: u8,
    request: AdminRequestType,
}


pub(crate) enum PlayerMessageType {

}

pub(crate) struct PlayerMessage {
    msg_type: PlayerMessageType,
}