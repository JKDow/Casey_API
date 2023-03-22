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
use rand::seq::SliceRandom;

#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Meld {
    rank: u32,
    cards: Vec<Card>,
    natural: bool,
}

impl Meld {

}

#[derive(Debug)]
struct Player {
    id: usize,
    hand: Vec<Card>,
    melds: Vec<Meld>,
    temp: Vec<Card>,
}

impl Player {
    fn new(id: usize) -> Player {
        Player { 
            id,
            hand: vec![],
            melds: vec![], 
            temp: vec![],
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
    }

    pub fn draw_deck(&mut self) {
        let card = self.deck.pop();
        match card {
            Some(card) => {
                self.players[self.player_turn].hand.push(card);
            }
            None => {
                //End game
            }
        }
    }

    pub fn take_pack(&mut self) {}

    pub fn print_hand(&mut self) {
        println!("Hand of player {}", self.player_turn);
        let hand = &self.players[self.player_turn].hand;
        for (i, card) in hand.iter().enumerate() {
            println!("{}: {:?}", i, card);
        }
    }

    pub fn throw(&mut self, card: usize) {
        assert!(card < self.players[self.player_turn].hand.len());
        self.discard.push(self.players[self.player_turn].hand.remove(card));
        self.player_turn += 1; 
        self.player_turn %= self.players.len();
    }
}

#[cfg(test)]
mod tests;
