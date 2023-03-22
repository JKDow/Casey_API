/* 
A game engine API for casey 
The engine will play canasta and enforce the rule 

On a branch for a simple implementation for solo hand with 2 canastas to win

Author: Joshua Dowling
Created: 21/03/23
Last Edited: 22/03/23
*/

#![allow(dead_code)]
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
}

impl Player {
    fn new(id: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: vec![], 
        }
    }
}


pub struct Game {
    deck: Vec<Card>,
    discard: Vec<Card>, 
    players: Vec<Player>, //first player to play (Player after "Dealer") is player 0
    player_turn: usize,
}

impl Game {
    pub fn new(number_of_players: usize) -> Game {
        let mut players: Vec<Player> = Vec::new();
        for i in 0..number_of_players {
            players.push(Player::new(i));
        }
        Game {
            deck: Game::make_deck(),
            discard: vec![],
            players,
            player_turn: 0,
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
