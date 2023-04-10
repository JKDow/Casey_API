/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use crate::game::players::Player;
use crate::setup::GameSettings;
use crate::game::cards::*;
use crate::game::thread::messages::{GameMessageType, GameMessage, PlayerMessage, AdminMessage, AdminReply};

enum GameState {
    PreGame, //game has not started yet
    NewTurn, //send data out to players and go next state
    WaitingForDraw, //wait for player response 
    UpdateDraw, //draw has been chosen, msg players and wait for action
    WaitingForAction, //wait for player response with meld or throw - update other players with any action 
    Finished,   //game ended
}

pub struct Game {
    pub(crate) game_id: u32,

    running: bool,
    points_to_win: u16,
    canastas_out: u8, 
    team_size: u8,

    game_state: GameState,
    current_player: u8,

    pub(crate) players: Vec<Option<Player>>, //vec of players - is option to allow for empty slots
    hand_backups: Vec<Vec<Card>>, //vec of vecs of cards - one vec of cards for each player

    deck: Vec<Card>,    
    discard: Vec<Card>,
    melds: Vec<Vec<Meld>>, //vec of vecs of melds - one vec of melds for each team 
    
    tx_backup: mpsc::Sender<GameMessage>,
    rx: mpsc::Receiver<GameMessage>, //msg in 
    player_tx: Vec<mpsc::Sender<PlayerMessage>>, //msg out to players

    admin_in: mpsc::Receiver<AdminMessage>, //msg in from admin
    admin_out: mpsc::Sender<AdminReply>, //msg out to admin
}

impl Game {
    /* 
    Name: new()
    Description: Creates a new game struct
    Params: 
        settings: &GameSettings - the settings for the game
        msg: (mpsc::Sender<AdminReply>, mpsc::Receiver<AdminMessage>) - the message channels for the admin
    Returns: Game - the game struct
    */
    pub(crate) fn new (settings: &GameSettings, msg: (mpsc::Sender<AdminReply>, mpsc::Receiver<AdminMessage>)) -> Game {
        //create unique game ID
        let game_id = rand::random();
        //setup sender and receiver for admin
        let (tx_admin, rx_admin) = mpsc::channel();
        //setup playrers and load with coms
        let mut players: Vec<Player> = Vec::new();
        let mut senders: Vec<mpsc::Sender<PlayerMessage>> = Vec::new();
        for i in 0..settings.num_players {
            let (tx_player, rx_player) = mpsc::channel();
            senders.push(tx_player);
            players.push(Player::new(game_id, i, i % settings.team_size, (tx_admin.clone(), rx_player)));
        }
        //create deck and deal to players 
        let mut deck = make_deck();
        for _ in 0..settings.deal_size {
            for i in 0..players.len() {
                players[i].hand.push(deck.pop().unwrap())
            }
        }
        //backup player hands and wrap players for loading into game struct
        let mut wrapped: Vec<Option<Player>> = vec![];
        let mut hand_backups: Vec<Vec<Card>> = vec![];
        for i in 0..players.len() {
            hand_backups.push(players[i].hand.clone());
            wrapped.push(Some(players.remove(0)));
        }

        Game {
            game_id,
            running: false,
            points_to_win: settings.points_to_win,
            canastas_out: settings.canastas_out,
            team_size: settings.team_size,
            game_state: GameState::PreGame,
            current_player: 0,

            players: wrapped,
            hand_backups: Vec::new(),
            deck,
            discard: Vec::new(),
            melds: Vec::new(),

            tx_backup: tx_admin,
            rx: rx_admin,
            player_tx: senders,

            admin_in: msg.1,
            admin_out: msg.0,
        }
    }

    /* 
    Name: start()
    Description: Starts the game and contains the main loop 
    Params:
        None
    Returns: thread::JoinHandle<()> - the thread handle
    */
    pub(crate) fn start(mut self) -> thread::JoinHandle<()>{
        self.running = true;    //update running
        let card = self.deck.pop().expect("No cards left in deck"); //get a card from the deck
        self.discard.push(card); //add it to the discard pile

        //if the card is a wild or a 3, keep drawing until it isn't
        while self.discard.last().unwrap().is_wild() || self.discard.last().unwrap().rank == 3 {
            let card = self.deck.pop().expect("No cards left in deck"); 
            self.discard.push(card); 
        }
        //msg all players the game has started and a copy of the discard pile 
        for i in 0..self.players.len() {
            let success = self.player_tx[i].send(PlayerMessage::GameStarted(self.discard.clone()));
            if let Err(e) = success {
                self.fix_player(i as u8);
                self.player_tx[i].send(e.0).expect("Failed to send message to player during start after fixing");
            }
        }

        thread::spawn(move || {
            while self.running {
                //handle admin messages
                match self.admin_in.try_recv() {
                    Ok(msg) => {
                        //handle the message
                    }
                    Err(e) => {
                        if e == TryRecvError::Disconnected {
                            panic!("Admin disconnected");   //Maybe handle better?
                        }
                    }
                }
                //handle player requests
                match self.rx.try_recv() {
                    Ok(msg) => {
                        //handle the message
                    }
                    Err(e) => {
                        if e == TryRecvError::Disconnected {
                            for i in 0..self.players.len() {
                                self.fix_player(i as u8);
                            }
                        }
                    }
                }
                
                //handle the game state 
                match self.game_state {
                    GameState::PreGame => {}
                    GameState::NewTurn => {}
                    GameState::WaitingForDraw => {}
                    GameState::UpdateDraw => {}
                    GameState::WaitingForAction => {}
                    GameState::Finished => {}
                }
            }
        })
    }

    fn fix_player(&mut self, player_id: u8) {

        println!("Player {} disconnected", player_id);

        let (tx_player, rx_player) = mpsc::channel();
        let mut player = Player::new(self.game_id, player_id, player_id % self.team_size, (self.tx_backup.clone(), rx_player));
        player.set_hand(self.hand_backups[player_id as usize].clone());
        self.player_tx[player_id as usize] = tx_player;
        self.players[player_id as usize] = Some(player)
    }
}

/* 
Name: make_deck()
Description: helper function that creates the deck 
Params:
    None
Returns: Vec<Card> - the deck
*/
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
