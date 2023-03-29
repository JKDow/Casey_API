#[derive(Debug, PartialEq)]
pub enum Suit {
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
    pub(crate) fn new(suit: Suit, rank: u32) -> Card {
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
    rank: u32,
    cards: Vec<Card>,
    natural: bool,
}

impl Meld {
    //change to a vector of melds to see if the combination is valid 
    //extra could be the top card of the discard 
    fn is_valid(cards: &Vec<Card>, extra: Option<Card>) -> bool { 
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

    fn can_take_pack(cards: &Vec<Card>, discard: &Card) -> bool {
        if discard.is_wild() || discard.rank == 3 {
            return false; 
        }
        todo!();
    }

    //if you cant add the card it will give it back
    //wild do is where you want wilds
    fn add(card: Card, wild_to: Option<usize>) -> Option<Card> { 
        todo!();
    }

    fn remove() -> Option<Card> {
        todo!();
    }

    fn clear() -> Vec<Card> {
        todo!();
    }


}