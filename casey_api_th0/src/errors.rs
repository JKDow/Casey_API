use::std::fmt;

#[derive(Debug)]
pub enum SettingsErrorType {
    InvalidNumberOfPlayers,
    InvalidTeamSize,
    InvalidPlayerRatio,
    InvalidCanastaOut
}

#[derive(Debug)]
pub struct SettingsError {
    pub error_type: SettingsErrorType,
    message: String,
}

impl SettingsError {
    pub fn new(error_type: SettingsErrorType, message: String) -> SettingsError {
        SettingsError {
            error_type,
            message,
        }
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

