/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use crate::game::thread::core::Game;
use crate::setup::GameSettings;
use crate::errors::{PlayerError, PlayerErrorType};
use crate::game::players::Player;

pub struct Admin {
    game_file: Option<Game>
}

impl Admin {
    pub fn new(settings: &GameSettings) -> Admin {
        Admin {
            game_file: Some(Game::new(settings))
        }
    }

    pub fn running(&self) -> bool {
        todo!();
    }

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
}


