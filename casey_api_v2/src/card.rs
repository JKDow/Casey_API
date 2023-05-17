
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Rank {
    Ace, Two,
    Three, Four,
    Five, Six,
    Seven, Eight,
    Nine, Ten,
    Jack, Queen,
    King, Joker
}

#[derive(Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

pub struct Card {
    pub (crate) id: usize,
    pub (crate) rank: Rank,
    pub (crate) suit: Suit,
    pub (crate) value: u8,
}

impl Card {
    pub(crate) fn new(id: usize, rank: Rank, suit: Suit) -> Card {
        let mut value = match rank {
            Rank::Ace | Rank::Two => 20,
            Rank::Three => {
                match suit {
                    Suit::Spades | Suit::Clubs => 5,
                    Suit::Hearts | Suit::Diamonds => 100,
                }
            }
            Rank::Four | Rank::Five | Rank::Six | Rank::Seven => 5,
            Rank::Joker => 50,
            _ => 10,
        };
        Card {
            id,
            rank,
            suit,
            value,
        }
    }

    pub(crate) fn is_normal(&self) -> bool {
        match self.rank {
            Rank::Ace | Rank::Four | Rank::Five | 
            Rank::Six | Rank::Seven | Rank::Eight | 
            Rank::Nine | Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => true,
            _ => false,
        }
    }

    pub(crate) fn is_wild(&self) -> bool {
        match self.rank {
            Rank::Two | Rank::Joker => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
    
    pub fn get_rank(&self) -> Rank {
        self.rank
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }
}

