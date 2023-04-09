/* 
Holds the game struct which contains the core game logic
Is the central server players communicate with
Acts as the admin command block aswel
*/

use crate::players::Player;
use crate::setup::GameSettings;

//contains players, deck, discard, melds, msg channels, tread handles
pub struct Game {
    players: Vec<Player>,
    deck: String,
    discard: String,
    //melds
    //msg in
    //msg out vec
    //thread handles
}

impl Game {
    pub(crate) fn new (settings: &GameSettings) -> Game {
        Game {
            players: Vec::new(),
            deck: String::new(),
            discard: String::new(),
        }
    }
}