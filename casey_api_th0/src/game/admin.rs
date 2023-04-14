/*
Holds the game struct which contains the core game logic until released to start tge fane
Works as the command block for the game to send commands to the game thread 
If the admin object is dropped the game will end as the game thread will stop and drop itself 
*/

use std::thread;
use std::sync::mpsc;
use crate::game::thread::core::Game;
use crate::setup::GameSettings;
use crate::errors::{PlayerError, PlayerErrorType, AdminError, AdminErrorType};
use crate::game::players::Player;
use crate::game::thread::messages::{AdminMessage, AdminReply};

pub struct Admin {
    game_file: Option<Game>,
    handle: Option<thread::JoinHandle<()>>,

    tx: mpsc::Sender<AdminMessage>, 
    rx: mpsc::Receiver<AdminReply>,
}

impl Admin {   
    /* 
    Name: new()
    Description: Creates a new admin struct
    Params: 
        settings: &GameSettings - the settings for the game
    Returns: Admin - the admin struct
    */
    pub fn new(settings: &GameSettings) -> Admin {
        let (admin_send, game_receive) = mpsc::channel();
        let (game_send, admin_receive) = mpsc::channel();
        Admin {
            game_file: Some(Game::new(settings, (game_send, game_receive))),
            handle: None,

            tx: admin_send,
            rx: admin_receive,
        }
    }

    /* 
    Name: take_player()
    Description: Takes a player from the game struct if it exists
        If the game struct is not in the admin struct then it sends a message to the game thread
    Params:
        player_num: u8 - the player number to take
    Returns: Result<Player, PlayerError> - the player struct if it exists or an error detailing why it doesn't
    */
    pub fn take_player(&mut self, player_num: u8) -> Result<Player, PlayerError> {
        match self.game_file {
            Some(ref mut game) => {
                //The game is still inside the admin struct
                game.take_player(player_num)
            }
            None => {
                //msg game
                todo!();
            }
        }
    }

    /* 
    Name: insert_player()
    Description: Inserts a player into the game struct if it exists
        If the game struct is not in the admin struct then it sends a message to the game thread
    Params:
        player: Player - the player struct to insert
    Returns: Result<(), PlayerError> - an error if the player already exists or the player is from the wrong game
    */
    pub fn insert_player(&mut self, player: Player) -> Result<(), PlayerError> {
        match self.game_file {
            Some(ref mut game) => {
                //The game is still inside the admin struct
                //check if game id in player matches game id 
                game.insert_player(player)
            }
            None => {
                //msg game
                todo!();
            }
        }
    }

    /* 
    Name: start()
    Description: Starts the game if it exists
        If the game struct is not in the admin struct then return an error
        Starting the game moves the game struct out of the admin struct and into the game thread
    Params:
        None
    Returns: Result<(), AdminError> - an error if the game is already running
    */
    pub fn start(&mut self) -> Result<(), AdminError> {
        match self.game_file.take() {
            Some(mut game) => {
                let handle = game.start();
                self.handle = Some(handle);
                Ok(())
            }
            None => {
                return Err(AdminError::new(AdminErrorType::GameAlreadyRunning, "Game already running"));
            }
        }
    }
}


