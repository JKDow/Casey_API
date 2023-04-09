/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use crate::errors::PlayerError;
use crate::errors::PlayerErrorType;
use crate::game::players::Player;
use crate::setup::GameSettings;
use crate::game::cards::*;
//contains players, deck, discard, melds, msg channels, tread handles
pub struct GameAdmin {
    game_id: u32,

    running: bool,
    points_to_win: u16,

    pub(crate) players: Vec<Option<Player>>, //vec of players - is option to allow for empty slots
    deck: Vec<Card>,    
    discard: Vec<Card>,
    melds: Vec<Vec<Meld>>, //vec of vecs of melds - one vec of melds for each team 
    //msg in
    //msg out vec
    //thread handles
}

impl GameAdmin {
    pub(crate) fn new (settings: &GameSettings) -> GameAdmin {
        let game_id = rand::random();
        let mut players: Vec<Player> = Vec::new();
        for i in 0..settings.num_players {
            players.push(Player::new(game_id, i, i % settings.team_size));
        }

        let mut deck = make_deck();
        for _ in 0..settings.deal_size {
            for i in 0..players.len() {
                players[i].hand.push(deck.pop().unwrap())
            }
        }

        let mut wrapped: Vec<Option<Player>> = vec![];
        for _ in 0..players.len() {
            wrapped.push(Some(players.remove(0)));
        }

        GameAdmin {
            game_id,
            running: false,
            points_to_win: settings.points_to_win,
            players: wrapped,
            deck,
            discard: Vec::new(),
            melds: Vec::new(),
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn take_player(&mut self, player_num: u8) -> Result<Player, PlayerError> {
        if player_num >= self.players.len() as u8 {
            return Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player number out of range"));
        }
        let player = self.players[player_num as usize].take();
        match player {
            Some(player) => Ok(player),
            None => Err(PlayerError::new(PlayerErrorType::InvalidPlayerNumber, "Player not found")),
        }
    }

    pub fn insert_player(&mut self, player: Player) -> Result<(), PlayerError> {
        //check if game id in player matches game id 
        if player.game_id != self.game_id {
            return Err(PlayerError::new(PlayerErrorType::PlayerFromWrongGame, "Player from wrong game"));
        }
        if self.players[player.player_num as usize].is_some() {
            return Err(PlayerError::new(PlayerErrorType::PlayerAlreadyInserted, "Player already exists"));
        }
        let num = player.player_num as usize;
        self.players[num] = Some(player);
        Ok(())
    }

}

//helper function that creates the deck 
fn make_deck() -> Vec<Card> { 
    use rand::prelude::*;
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