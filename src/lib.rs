/* 
A game engine API for casey 
The engine will play canasta and enforce the rule 
Author: Joshua Dowling
Created: 21/03/23
Last Edited: 22/03/23
*/



/* 
implement game_setting trait with get methods for everything 
implement trait for game too

different structs for games and hands 
different structs for solo and team? Probably not 
    Different settings structs for each though 


Seperate settings structs for solo and team 
Different game struct for game and hand 
game struct can contain hand struct? 
*/

#![allow(dead_code)]

use std::fmt::Error;
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

struct Card {
    suit: Suit, 
    rank: u32,  // 0 is joker, 1 is ace - 13 is king
    value: u32, //how many points is the card worth 
}

impl Card {
    fn new(suit: Suit, rank: u32) -> Card {
        let value = 
            if rank == 0 { 50 } 
            else if rank <= 2 { 20 }
            else if rank == 3 {
                match suit {
                    Suit::Heart | Suit::Diamond => 100,
                    Suit::Spade | Suit::Club => 5,
                }
            }
            else if rank < 8 { 5 }
            else { 10 };
        Card {
            suit,
            rank,
            value,
        }
    }
}

struct Meld {
    rank: u32,
    cards: Vec<Card>,
    natural: bool,
}

impl Meld {
    //new

}

struct Player {
    id: usize,
    hand: Vec<Card>,
    melds: Vec<Meld>,
    team_number: usize,
}

impl Player {
    fn new(id: usize, team_number: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: vec![], 
            team_number 
        }
    }
}

pub struct GameSettings {
    players_per_team: usize,
    number_of_teams: usize,
    points_first_meld: usize,
    out_canastas: usize,
    points_to_win: usize, //set to 0 if just a hand
    first_meld_increment: usize,
} 

impl GameSettings {
    pub fn new(players: usize) -> GameSettings {
        GameSettings {
            players_per_team: 1,
            number_of_teams: players,
            points_first_meld: 50,
            out_canastas: 2,
            points_to_win: 0,
            first_meld_increment: 0,
        }
    }

    pub fn set_teams(&mut self, teams: usize) -> Result<usize,Error> {





        return Ok(0);
    }
}

pub struct Game {
    deck: Vec<Card>,
    discard: Vec<Card>, 
    players: Vec<Player>, //first player to play (Player after "Dealer") is player 0
    player_turn: usize,
    round_number: usize,

    parameters: GameSettings,
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        let mut players: Vec<Player> = Vec::new();
        let mut team_num = 0; 
        for i in 0..(settings.number_of_teams * settings.players_per_team) {
            players.push(Player::new(i, team_num));
            team_num += 1;
            if team_num == settings.number_of_teams {
                team_num = 0;
            }
        }
        Game {
            deck: Game::make_deck(),
            discard: vec![],
            players,
            player_turn: 0,
            round_number: 0,
            parameters: settings
        }
    }

    fn make_deck() -> Vec<Card> {
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
        return deck;
    }
}

#[cfg(test)]
mod tests;
