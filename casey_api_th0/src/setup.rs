//a struct for canasta game settings
//number of players, team size, canastas to go out, game vs hand

use crate::errors::{SettingsErrorType, SettingsError};
use crate::game::Game;

pub struct GameSettings {
    num_players: u8,
    team_size: u8,
    canastas_out: u8,
    first_meld: u8,
    meld_increment: u8,
    points_to_win: u16,
    full_game: bool,
}

impl GameSettings {
    pub fn new(num_players: u8, team_size: u8, canastas_out: u8) -> GameSettings {
        GameSettings {
            num_players,
            team_size,
            canastas_out,
            first_meld: 50,
            meld_increment: 0,
            points_to_win: 0,
            full_game: false,
        }
    }

    pub fn set_players(&mut self, num_players: u8) {
        self.num_players = num_players;
    }

    pub fn set_team_size(&mut self, team_size: u8) {
        self.team_size = team_size;
    }

    pub fn set_canastas_out(&mut self, canastas_out: u8) {
        self.canastas_out = canastas_out;
    }

    pub fn set_first_meld(&mut self, first_meld: u8) {
        self.first_meld = first_meld;
    }

    pub fn set_full_game(&mut self, meld_increment: u8, points_to_win: u16) {
        self.full_game = true; 
        self.meld_increment = meld_increment;
        self.points_to_win = points_to_win;
    }

    pub fn set_hand(&mut self) {
        self.full_game = false;
    }

    //checks that the number of players devides into the number of teams
    //checks that the number of players is greater than 1
    //checks that the number of canastas to go out is greater than 0
    //checks that the number of canastas to go out is less than 13
    pub fn confirm_settings(&self) -> Result<Game, SettingsError> {
        if self.team_size == 0 {
            return false;
        } else if self.num_players < 2 {
            return false;
        } else if self.num_players % self.team_size != 0 {
            return false;
        } else if self.canastas_out > 11 {
            return false;
        } else {
            return true;
        } 
    }
}