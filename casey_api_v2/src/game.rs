/* 
The game struct for canasta 

Development Plans: 
    - Create game struct
*/

use crate::player::Player;
use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(PartialEq)]
enum TurnPhase {
    PreGame,
    Draw,
    Discard,
}

struct Game {
    players: Vec<Player>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    discard_frozen: bool,
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
            discard_frozen: false,
            player_turn: 0,
            turn_phase: TurnPhase::PreGame,
        };
        for i in 0..num_players {
            g.players.push(Player::new(i));
        }
        return g;
    }

    /// ## Description
    /// Deals cards to each player\n 
    /// Will deal to player 0 first 
    /// ## Parameters
    /// * num: usize - the number of cards to deal to each player
    /// ## Returns
    /// Result<(), ()> - Ok(()) if successful, Err(()) if not
    /// * Err(()) if it is not the pregame phase
    /// * Err(()) if the number of cards to deal is greater than the number of cards in the deck divided by the number of players
    pub fn deal(&mut self, num: usize) -> Result<(), ()> {
        if self.turn_phase != TurnPhase::PreGame {
            return Err(());
        }
        if num > self.deck.len() / self.players.len() {
            return Err(());
        }
        for _ in 0..num {
            for player in self.players.iter_mut() {
                player.hand.push(self.deck.pop().unwrap());
            }
        }
        self.discard.push(self.deck.pop().unwrap());
        while self.discard.last().unwrap().is_normal() == false {
            self.discard.push(self.deck.pop().unwrap());
        }
        self.turn_phase = TurnPhase::Draw;
        Ok(())
    }

    /// ## Description
    /// Draws a card from the deck and adds it to the player's hand\n
    /// Will also change the turn phase to discard
    /// ## Parameters
    /// * player_num: usize - the player who is drawing
    /// ## Returns
    /// Result<(), ()> - Ok(()) if successful, Err(()) if not
    /// * Err(()) if it is not the player's turn
    /// * Err(()) if it is not the draw phase
    /// * Err(()) if the player has red threes in hand
    pub fn draw(&mut self, player_num: usize) -> Result<(), ()> {
        if player_num != self.player_turn {
            return Err(());
        }
        if self.turn_phase != TurnPhase::Draw {
            return Err(());
        }
        if self.players[self.player_turn].check_threes() > 0 {
            return Err(());
        }
        let card = self.deck.pop().unwrap();
        self.players[self.player_turn].hand.push(card);
        self.players[self.player_turn].sort_hand();
        self.turn_phase = TurnPhase::Discard;
        Ok(())
    }

    /// ## Description
    /// Discards a card from the player's hand and adds it to the discard pile\n
    /// Will also change the turn phase to draw
    /// ## Parameters
    /// * player_num: usize - the player who is discarding
    /// * card_id: usize - the id of the card to discard
    /// ## Returns
    /// Result<(), ()> - Ok(()) if successful, Err(()) if not
    /// * Err(()) if it is not the player's turn
    /// * Err(()) if it is not the discard phase
    /// * Err(()) if the player does not have the card
    /// ## TODO
    /// * Add going out logic
    pub fn discard(&mut self, player_num: usize, card_id: usize) -> Result<(), ()> {
        if player_num != self.player_turn {
            return Err(());
        }
        if self.turn_phase != TurnPhase::Discard {
            return Err(());
        }
        let card = match self.players[self.player_turn].remove_card(card_id) {
            Some(c) => c,
            None => return Err(()),
        };
        if card.is_wild() {
            self.discard_frozen = true;
        }
        self.discard.push(card);
        self.player_turn = (self.player_turn + 1) % self.players.len();
        self.turn_phase = TurnPhase::Draw;
        Ok(())
    }

    /// ## Description
    /// Melds cards from the player's hand\n
    /// If the meld fails it will add the cards back
    /// ## Parameters
    /// * player_num: usize - the player who is melding
    /// * card_ids: Vec<usize> - the ids of the cards to meld
    /// * suit: Suit - the suit of the meld
    /// ## Returns
    /// Result<(), ()> - Ok(()) if successful, Err(()) if not
    /// * Err(()) if it is not the player's turn
    /// * Err(()) if it is not the discard phase
    /// * Err(()) if the player does not have the cards
    /// * Err(()) if the meld is invalid
    /// ## TODO
    /// * Add going out logic 
    pub fn meld(&mut self, player_num: usize, card_ids: Vec<usize>, suit: Suit) -> Result<(), ()> {
        // error check turn and phase 
        if player_num != self.player_turn {
            return Err(());
        }
        if self.turn_phase != TurnPhase::Discard {
            return Err(());
        }
        // create vector of cards from IDs
        let mut cards = Vec::new();
        for id in card_ids {
            let card = match self.players[self.player_turn].remove_card(id) {
                Some(c) => c,
                None => {
                    // if theres an error then add cards back
                    self.players[self.player_turn].add_cards(cards);
                    return Err(());
                }
            };
            cards.push(card);
        }
        // try to meld cards
        if let Err(e) = self.players[self.player_turn].meld(cards, suit) {  
            // if theres an error then add cards back
            self.players[self.player_turn].add_cards(e);
            return Err(())
        }
        Ok(())
    }

    pub fn meld_threes(&mut self, player_num: usize) -> Result<Vec<usize>, ()> {
        if player_num != self.player_turn {
            return Err(());
        }
        if self.turn_phase != TurnPhase::Draw {
            return Err(());
        }
        let mut num = self.players[player_num].meld_red_threes();
        let mut card_ids = Vec::new();
        while num > 0 {
            let card = self.deck.pop().unwrap();
            if card.is_red_three() {
                card_ids.push(card.id);
                self.players[player_num].red_threes.push(card);
            } else {
                self.players[player_num].hand.push(card);
                num -= 1;
            }
        }
        return Ok(card_ids);
    }

    pub fn take_pack(&mut self, player_num: usize, card_ids: Vec<usize>) -> Result<(), ()> {
        if player_num != self.player_turn {
            return Err(());
        }
        if self.turn_phase != TurnPhase::Draw {
            return Err(());
        }
        if self.players[player_num].check_threes() > 0 {
            return Err(());
        }
        // check top card
        let top_card = self.discard.last().unwrap();
        if !top_card.is_normal() {
            return Err(()); // top card is not normal and pack cannot be taken
        }
        // create vector of cards from ids 
        let mut cards = Vec::new();
        for id in card_ids {
            match self.players[self.player_turn].remove_card(id) {
                Some(c) => {
                    if c.is_normal() {
                        if c.suit != top_card.suit {
                            cards.push(c);
                            self.players[self.player_turn].add_cards(cards);
                            return Err(()); //suits didnt match 
                        }
                        cards.push(c);
                    } else if c.is_wild() {
                        cards.push(c);
                    } else {
                        cards.push(c);
                        self.players[self.player_turn].add_cards(cards);
                        return Err(()); //card was a three 
                    }
                }
                None => {
                    // if theres an error then add cards back
                    self.players[self.player_turn].add_cards(cards);
                    return Err(()); //doesnt own card
                }
            };
        }
        // check if player has a meld of the same suit
        match self.players[self.player_turn].melds.get_mut(&top_card.suit) {
            // has melds
            Some(meld) => { 
                if self.discard_frozen == false {
                    cards.push(self.discard.pop().unwrap());
                    if let Err(mut e) = meld.add_cards(cards) {
                        self.discard.push(e.pop().unwrap());    // return top card to discard 
                        self.players[self.player_turn].add_cards(e);
                        return Err(())
                    }
                    // take the pack
                    for card in self.discard.drain(..) {
                        self.players[self.player_turn].hand.push(card);
                    }
                    self.players[self.player_turn].sort_hand();
                } else {    // pack not frozen 
                    let mut counter = 0;
                    for card in &cards {
                        if !card.is_wild() {
                            counter += 1;
                        }
                    }  
                    // when the pack is frozen you need 2 cards to take 
                    if counter < 2 {
                        self.players[self.player_turn].add_cards(cards);
                        return Err(())  //doesnt have 2 cards for frozen deck 
                    }
                    cards.push(self.discard.pop().unwrap());
                    if let Err(mut e) = meld.add_cards(cards) {
                        self.discard.push(e.pop().unwrap());    // return top card to discard 
                        self.players[self.player_turn].add_cards(e);
                        return Err(())  //invalid meld
                    }
                    // if here take the pack 
                    // move discard cards to players hand
                    for card in self.discard.drain(..) {
                        self.players[self.player_turn].hand.push(card);
                    }
                    self.players[self.player_turn].sort_hand();
                    self.discard_frozen = false;
                }
            }
            // does not have melds 
            None => {
                let suit = top_card.suit;
                cards.push(self.discard.pop().unwrap());
                if let Err(mut e) = self.players[self.player_turn].meld(cards, suit) {
                    self.discard.push(e.pop().unwrap());    // return top card to discard 
                    self.players[self.player_turn].add_cards(e);
                    return Err(())  //invalid meld
                }
                // if here take the pack
                // move discard cards to players hand
                for card in self.discard.drain(..) {
                    self.players[self.player_turn].hand.push(card);
                }
                self.players[self.player_turn].sort_hand();
                self.discard_frozen = false;
            }
        }
        Ok(())
    }
}