/* 
The game struct for canasta 

Development Plans: 
    - Create game struct
*/

use crate::player::Player;
use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

enum TurnPhase {
    Draw,
    Discard,
}

struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    player_turn: usize,
    turn_phase: TurnPhase,
}

impl Game {
    /// ## Description
    /// Creates a new game with the specified number of players\n
    /// Will also create the deck and shuffle it
    /// ## Parameters
    /// * num_players: usize - the number of players in the game
    /// ## Returns
    /// * Game - the new game
    /// ## Example
    /// ```
    /// let g = Game::new(4);
    /// ```
    ///  
    pub fn new(num_players: usize) -> Game {
        fn make_deck() -> Vec<Card> { //helper function that creates the deck 
            let mut deck = Vec::new();
            let suits = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
            let ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Joker];
            let mut id = 0;
            for suit in suits.iter() {
                for rank in ranks.iter() {
                    deck.push(Card::new(id, *rank, *suit));
                    id += 1;
                }
            }
            let mut rng = rand::thread_rng();
            deck.shuffle(&mut rng);
            return deck;
        }
        let mut g = Game {
            players: Vec::new(),
            deck: make_deck(),
            discard: Vec::new(),
            player_turn: 0,
            turn_phase: TurnPhase::Draw,
        };
        for i in 0..num_players {
            g.players.push(Player::new(i));
        }
        return g;
    }

    fn deal(&mut self, num: usize) -> Result<>{
        for _ in 0..num {
            for player in self.players.iter_mut() {
                player.hand.push(self.deck.pop().unwrap());
            }
        }
        self.discard.push(self.deck.pop().unwrap());
    }
}