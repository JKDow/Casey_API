/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
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
                if player_num >= game.players.len() as u8 {
                    return Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player number out of range"));
                }
                let player = game.players[player_num as usize].take();
                match player {
                    Some(player) => Ok(player),
                    None => Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player not found")),
                }
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
                if player.game_id != game.game_id {
                    return Err(PlayerError::new(PlayerErrorType::PlayerFromWrongGame, "Player from wrong game"));
                }
                if game.players[player.player_num as usize].is_some() {
                    return Err(PlayerError::new(PlayerErrorType::PlayerAlreadyInserted, "Player already exists"));
                }
                let num = player.player_num as usize;
                game.players[num] = Some(player);
                Ok(())
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


