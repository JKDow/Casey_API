use std::fmt;
use crate::game::cards::Card;

#[derive(Debug)]
pub enum TurnErrorType {
    NotThrowPhase,
    NotDrawPhase(Card), 
    InvalidCard,
}
#[derive(Debug)]

pub struct TurnError {
    error_type: TurnErrorType,
    msg: String,
}

impl fmt::Display for TurnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl TurnError {
    pub(crate) fn invalid_card(msg: &str) -> TurnError {
        TurnError { error_type: TurnErrorType::InvalidCard, msg: String::from(msg) }
    }
    pub(crate) fn not_draw_phase(msg: &str, card: Card) -> TurnError {
        TurnError { error_type: TurnErrorType::NotDrawPhase(card), msg: String::from(msg) }
    }
    pub(crate) fn not_throw_phase(msg: &str) -> TurnError {
        TurnError { error_type: TurnErrorType::NotThrowPhase, msg: String::from(msg) }
    }
}