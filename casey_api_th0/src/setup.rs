//a struct for canasta game settings
//number of players, team size, canastas to go out, game vs hand

use crate::errors::{SettingsErrorType, SettingsError};
use crate::game::admin::GameAdmin;

pub struct GameSettings {
    num_players: u8,
    team_size: u8,
    canastas_out: u8,
    points_to_win: u16,
}

impl GameSettings {
    pub fn new(num_players: u8, team_size: u8, canastas_out: u8) -> GameSettings {
        GameSettings {
            num_players,
            team_size,
            canastas_out,
            points_to_win: 0,
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

    pub fn set_points_to_win(&mut self, points_to_win: u16) {
        self.points_to_win = points_to_win;
    }

    //checks that the number of players devides into the number of teams
    //checks that the number of players is greater than 1
    //checks that the number of canastas to go out is greater than 0
    //checks that the number of canastas to go out is less than 13
    pub fn confirm_settings(&self) -> Result<GameAdmin, SettingsError> {
        if self.team_size == 0 {
            return Err(SettingsError::new(SettingsErrorType::InvalidTeamSize, "Team size cannot be 0"));
        } else if self.num_players < 2 {
            return Err(SettingsError::new(SettingsErrorType::InvalidNumberOfPlayers, "Number of players must be greater than 1"));
        } else if self.num_players % self.team_size != 0 {
            return Err(SettingsError::new(SettingsErrorType::InvalidPlayerRatio, "Number of players must be divisible by team size"));
        } else if self.canastas_out > 11 {
            return Err(SettingsError::new(SettingsErrorType::InvalidCanastaOut, "Number of canastas to go out must be less than 12"));
        } else {
            return Ok(GameAdmin::new(self));
        } 
    }
}