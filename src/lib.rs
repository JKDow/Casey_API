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

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug)]
pub struct Card {
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

    fn is_wild(&self) -> bool {
        if self.rank == 0 || self.rank == 2 {
            return true;
        } else {
            return false;
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
    fn can_meld(cards: &Vec<Card>) -> bool {
        if cards.len() < 3 { return false; }
        let suit = &cards[0].suit;
        let mut wild_counter = 0; 
        for card in cards {
            if card.is_wild() {
                wild_counter += 1;
            } else if card.suit != *suit || card.rank == 3 {
                return false;
            }
        }
        if cards.len() - wild_counter < wild_counter {
            return false;
        }
        return true;
    } 

    fn can_take_pack(cards: &Vec<Card>, top_discard: &Card) -> bool {
        todo!();
    }
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
    frozen: bool,
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

    pub fn draw_deck(&mut self) { //draw top card from the deck
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

    pub fn take_pack(&mut self) -> bool { //try to make the current pack with the temp cards 
        todo!();
    }

    pub fn throw(&mut self, card: usize) { //throws card with 'card' index
        assert!(card < self.players[self.player_turn].hand.len());
        self.discard.push(self.players[self.player_turn].hand.remove(card));
        self.player_turn += 1; 
        self.player_turn %= self.players.len();
    }

    pub fn get_hand(&self) -> &Vec<Card> { //returns reference to hand of current player 
        return &self.players[self.player_turn].hand;
    }

    pub fn get_discard(&self) -> &Vec<Card> { //returns reference to discard pile 
        return &self.discard;
    }
}

pub mod prints {
    use super::Card;
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

#[cfg(test)]
mod tests;
