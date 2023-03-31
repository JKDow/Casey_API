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

pub enum TempMeldType{
    PushWild,
    InvalidCardNumber,
    InvalidRank,
    InvalidMeld,
}

pub struct TempMeldError {
    error_type: TempMeldType,
    msg: String
}

impl fmt::Display for TempMeldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl TempMeldError {
    pub(crate) fn wild(msg: &str) -> TempMeldError {
        TempMeldError { error_type: TempMeldType::PushWild, msg: String::from(msg) }
    }
    pub(crate) fn card_number(msg: &str) -> TempMeldError {
        TempMeldError { error_type: TempMeldType::InvalidCardNumber, msg: String::from(msg) }
    }
    pub(crate) fn rank(msg: &str) -> TempMeldError {
        TempMeldError { error_type: TempMeldType::InvalidRank, msg: String::from(msg) }
    }
    pub(crate) fn meld(msg: &str) -> TempMeldError {
        TempMeldError { error_type: TempMeldType::InvalidMeld, msg: String::from(msg) }
    }

    pub(crate) fn from(error: MeldError) -> TempMeldError {
        match error.error_type {
            MeldErrorType::InvalidRank => TempMeldError::rank(&error.msg),
            MeldErrorType::InvalidCard(_) => TempMeldError::card_number(&error.msg),
            MeldErrorType::TooManyWilds(_) => TempMeldError::wild(&error.msg),
            MeldErrorType::InvalidIndex => TempMeldError::card_number(&error.msg),
        }
    }
}

pub(crate) enum MeldErrorType {
    InvalidRank, 
    InvalidCard(Card),
    TooManyWilds(Card),
    InvalidIndex,
}

//create a new error type for melds and implement the display trait and creation functions 
pub struct MeldError {
    error_type: MeldErrorType,
    msg: String
}

impl fmt::Display for MeldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl MeldError {
    pub(crate) fn invalid_rank(msg: &str) -> MeldError {
        MeldError { error_type: MeldErrorType::InvalidRank, msg: String::from(msg) }
    }
    pub(crate) fn invalid_card(msg: &str, card: Card) -> MeldError {
        MeldError { error_type: MeldErrorType::InvalidCard(card), msg: String::from(msg) }
    }
    pub(crate) fn too_many_wilds(msg: &str, card: Card) -> MeldError {
        MeldError { error_type: MeldErrorType::TooManyWilds(card), msg: String::from(msg) }
    }
    pub(crate) fn invalid_index(msg: &str) -> MeldError {
        MeldError { error_type: MeldErrorType::InvalidIndex, msg: String::from(msg) }
    }

    pub(crate) fn get_card(&self) -> Option<Card> {
        match &self.error_type {
            MeldErrorType::InvalidCard(card) => Some(card.clone()),
            MeldErrorType::TooManyWilds(card) => Some(card.clone()),
            _ => None,
        }
    }
}



