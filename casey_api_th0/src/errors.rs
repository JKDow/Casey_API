use::std::fmt;
use crate::game::players::Player;

/*
Name: Settings Error
Description: Error type for conmfirming settings
Types: 
    InvalidNumberOfPlayers
    InvalidTeamSize
    InvalidPlayerRatio
    InvalidCanastaOut
    InvalidDealSize
*/
#[derive(Debug)]
pub enum SettingsErrorType {
    InvalidNumberOfPlayers,
    InvalidTeamSize,
    InvalidPlayerRatio,
    InvalidCanastaOut,
    InvalidDealSize,
}
#[derive(Debug)]
pub struct SettingsError {
    pub error_type: SettingsErrorType,
    message: String,
}
impl SettingsError {
    pub fn new(error_type: SettingsErrorType, message: &str) -> SettingsError {
        SettingsError {
            error_type,
            message: String::from(message)
        }
    }
}
impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

/* 
Namme: Player Error
Description: Error type for taking and inserting players 
Types: 
    InvalidPlayerNumber     - Player number out of bounds
    PlayerAlreadyTaken      - already been taken from vector
    PlayerFromWrongGame     - game ID does not match, returns player for retreival 
    PlayerAlreadyInserted   - somehow the player has been duplicated and is being inserted again, returns player for retreival
*/
#[derive(Debug)]
pub enum PlayerErrorType {
    InvalidPlayerNumber, 
    PlayerAlreadyTaken, 
    PlayerFromWrongGame(Player),  
    PlayerAlreadyInserted(Player),  
}
#[derive(Debug)]
pub struct PlayerError {
    pub error_type: PlayerErrorType,
    message: String,
}
impl PlayerError {
    pub fn new(error_type: PlayerErrorType, message: &str) -> PlayerError {
        PlayerError {
            error_type,
            message: String::from(message)
        }
    }
}
impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

/*
Name: Admin Error
Description: Error type for admin actions 
Types:  
    GameAlreadyRunning
*/
#[derive(Debug)]
pub enum AdminErrorType {
    GameAlreadyRunning,
}

pub struct AdminError {
    pub error_type: AdminErrorType,
    message: String,
}

impl AdminError {
    pub fn new(error_type: AdminErrorType, message: &str) -> AdminError {
        AdminError {
            error_type,
            message: String::from(message)
        }
    }
}

impl fmt::Display for AdminError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}