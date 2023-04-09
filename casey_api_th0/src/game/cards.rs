#[derive(Debug, PartialEq, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit, 
    pub (crate) rank: usize,  // 0 is joker, 1 is ace - 13 is king
    pub (crate) value: usize, //how many points is the card worth 
}

impl Card {
    pub(crate) fn new(suit: Suit, rank: usize) -> Card {
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
    
    //checks if the given card is a wild 
    pub(crate) fn is_wild(&self) -> bool {
        if self.rank == 0 || self.rank == 2 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
pub struct Meld {
    pub(crate) rank: usize,
    cards: Vec<Card>,
    natural: bool,
}