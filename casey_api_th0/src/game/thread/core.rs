/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Is only accessible through the admin and player structs 
*/

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use crate::game::players::Player;
use crate::setup::GameSettings;
use crate::game::cards::*;
use crate::game::thread::messages::{GameMessageType, GameMessage, PlayerMessage, AdminMessage, AdminReply};
use crate::errors::{PlayerError, PlayerErrorType};

enum GameState {
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
            game_state: GameState::NewTurn,
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
                //Handle admin messages
                self.handle_admin_request();
                //handle player requests
                self.handle_player_request();
                //handle the game state 
                self.run_game();
            }
        })
    }

    /* 
    Name: fix player
    Description: Fixes a player if they disconnect
    Params:
        player_id: u8 - the id of the player to fix
    Returns: None
    */
    fn fix_player(&mut self, player_id: u8) {

        println!("Player {} disconnected", player_id);

        let (tx_player, rx_player) = mpsc::channel();
        let mut player = Player::new(self.game_id, player_id, player_id % self.team_size, (self.tx_backup.clone(), rx_player));
        player.set_hand(self.hand_backups[player_id as usize].clone());
        self.player_tx[player_id as usize] = tx_player;
        self.players[player_id as usize] = Some(player)
    }

    /*
    Name: handle_admin_request()
    Description: 
        Receives and handles all messages from the admin 
        Admin gets priority and will have all their messages handled each time this is called
    Params: None
    Returns: None
    */
    fn handle_admin_request(&mut self) {
        let mut handle_admin = true; 
        let mut pause = false;
        while handle_admin {
            match self.admin_in.try_recv() {
                Ok(msg) => {
                    //handle the message
                    match msg {
                        AdminMessage::TakePlayer(id) => {
                            //attempt to take and reply to admin
                            let result = self.take_player(id);
                            self.msg_admin(AdminReply::TakePlayer(result));
                        }
                        AdminMessage::InsertPlayer(player) => {
                            //attempt to insert and reply to admin
                            let result = self.insert_player(player);
                            self.msg_admin(AdminReply::InsertPlayer(result));
                        }
                        AdminMessage::PauseGame => pause = true,  //pause the game by looping here
                        AdminMessage::ResumeGame => pause = false, //wait for this resume message 
                        AdminMessage::EndGame => {
                            //pass game object back to admin and end
                            self.running = false;
                            handle_admin = false;
                        }
                    }
                }
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                        panic!("Admin disconnected");   //Maybe handle better?
                    }
                    if pause == false {
                        handle_admin = false;
                    }
                }
            } 
        }
    }

    /* 
    Name: handle_player_request()
    Description: Receives and handles all messages from the players
    Params: None
    Returns: None
    */
    fn handle_player_request(&mut self) {
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
    }

    /* 
    Name: run_game()
    Description: Handles the game state and runs the game
    Params: None
    Returns: None
    */
    fn run_game(&mut self) {
        match self.game_state {
            GameState::NewTurn => {
                //msg the player whos turn it is with an action object for drawing
                //move to next state
            }
            GameState::WaitingForDraw => {
                //do nothing really
            }
            GameState::UpdateDraw => {}
            GameState::WaitingForAction => {}
            GameState::Finished => {}
        }
    }

    /* 
    Name: msg_admin()
    Description: Sends a message to the admin
    Params:
        msg: AdminReply - the message to send
    Returns: None
    */
    fn msg_admin(&mut self, msg: AdminReply) {
        self.admin_out.send(msg).expect("Failed to send message to admin"); //if the admin cant accept messages communcation is over
    }

    /* 
    Name: insert_player()
    Description: Inserts a player into the game
    Params:
        player: Player - the player to insert
    Returns: Result - Ok if the player was inserted, Err if the player was not inserted
    */
    pub(crate) fn insert_player(&mut self, player: Player) -> Result<(), PlayerError> {
        if player.game_id != self.game_id {
            return Err(PlayerError::new(PlayerErrorType::PlayerFromWrongGame(player), "Player from wrong game"));
        }
        if self.players[player.player_num as usize].is_some() {
            return Err(PlayerError::new(PlayerErrorType::PlayerAlreadyInserted(player), "Player already exists"));
        }
        let num = player.player_num as usize;
        self.players[num] = Some(player);
        Ok(())
    }

    /* 
    Name: take_player()
    Description: Takes a player from the game
    Params:
        player_id: u8 - the id of the player to take
    Returns: Result 
        Ok if the player was taken and contains the player
        Err if the player was not taken
    */
    pub(crate) fn take_player(&mut self, player_id: u8) -> Result<Player, PlayerError> {
        if player_id >= self.players.len() as u8 {
            return Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player number out of range"));
        }
        let player = self.players[player_id as usize].take();
        match player {
            Some(player) => Ok(player),
            None => Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player not found")),
        }
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
