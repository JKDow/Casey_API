/* 
A game engine API for casey 
The engine will play canasta and enforce the rule 

On a branch for a simple implementation for solo hand with 2 canastas to win
limited options with limited error checking

Author: Joshua Dowling
Created: 21/03/23
Last Edited: 22/03/23
*/

#![allow(dead_code)]

pub mod game {
    pub mod cards;
    pub mod player;

    use rand::seq::SliceRandom;
    use cards::*;
    use player::*;

    pub struct Game {
        deck: Vec<Card>,
        discard: Vec<Card>, 
        frozen: bool,
        pub players: Vec<Player>, //first player to play (Player after "Dealer") is player 0
        player_turn: usize,
    }

    impl Game {
        fn make_deck() -> Vec<Card> { //helper function that creates the deck 
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

        pub fn new(number_of_players: usize) -> Game {
            let deck = Game::make_deck();
            let mut players: Vec<Player> = Vec::new();
            for i in 0..number_of_players {
                players.push(Player::new(i));
            }
            Game {
                deck,
                discard: vec![],
                frozen: false,
                players,
                player_turn: 0,
            }
        }

        pub fn deal(&mut self, number_of_cards: usize) {
            if number_of_cards * self.players.len() >= self.deck.len() {
                panic!("Too many cards to deal");
            }
            for _ in 0..number_of_cards {
                for i in 0..self.players.len() {
                    self.players[i].hand.push(self.deck.pop().unwrap())
                }
            }
            self.discard.push(self.deck.pop().unwrap());
        }

        pub fn get_discard(&self) -> &Vec<Card> { //returns reference to discard pile 
            return &self.discard;
        }

        pub fn current_player(&self) -> usize {
            self.player_turn
        }

        pub fn get_player(&mut self, id: usize) -> &mut Player {
            return &mut self.players[id];
        }

        fn next_turn(&mut self) {
            self.player_turn += 1; 
            self.player_turn %= self.players.len();
            self.players[self.player_turn].turn_phase = player::TurnPhase::Draw;
        }

        pub fn to_discard(&mut self, card: Card) {
            self.discard.push(card);
            self.next_turn();        
        }
    }
}

pub mod prints {
    use super::game::cards::*;
    pub fn print_discard(cards: &Vec<Card>) {
        println!("Discard pile with most recent card printing first");
        for card in cards.iter().rev() {
            println!("{:?}", card);
        }
        println!("");
    }

    pub fn print_hand(hand: &Vec<Card>) {
        println!("Hand of player");
        for (i, card) in hand.iter().enumerate() {
            println!("{}: {:?}", i, card);
        }
        println!("");
    }
}

pub mod errors {
    use std::fmt;

    #[derive(Debug)]
    pub enum TurnErrorType {
        WrongGamePhase,
        GamesOver, 
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
        pub(crate) fn turn_phase(msg: &str) -> TurnError {
            TurnError { error_type: TurnErrorType::WrongGamePhase, msg: String::from(msg) }
        }
    }
}

#[cfg(test)]
mod tests;
