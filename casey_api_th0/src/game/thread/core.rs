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
use crate::game::thread::messages::{GameRequestType, GameRequest, PlayerMessageType, PlayerMessage};

//contains players, deck, discard, melds, msg channels, tread handles
pub struct Game {
    pub(crate) game_id: u32,

    running: RwLock<bool>,
    points_to_win: u16,

    pub(crate) players: Vec<Option<Player>>, //vec of players - is option to allow for empty slots
    hand_backups: Vec<Vec<Card>>, //vec of vecs of cards - one vec of cards for each player

    deck: Vec<Card>,    
    discard: Vec<Card>,
    melds: Vec<Vec<Meld>>, //vec of vecs of melds - one vec of melds for each team 
    
    rx: mpsc::Receiver<GameRequest>, //msg in 
    player_tx: Vec<mpsc::Sender<PlayerMessage>>, //msg out to players
    handle: Option<thread::JoinHandle<()>>
}

impl Game {
    pub(crate) fn new (settings: &GameSettings) -> Game {
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

        Game {
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

    pub fn start(&mut self) -> Result<(), AdminError>{
        {
            let mut running = self.running.write().unwrap();
            *running = true;
        }
        // let handler = thread::spawn(move || {
        //     loop {
        //         let running = self.running.read().unwrap();
        //         if !*running {
        //             break;
        //         }
        //         //drop(running);
        //     }
        // });

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
