use crate::errors::MeldError;

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

impl Meld {

    //creates a new meld with the given card
    //if the card is a 3 or a wild, returns an error containing the card
    pub(crate) fn new(card: Card) -> Result<Meld, MeldError> {
        if card.is_wild() || card.rank == 3  {
            return Err(MeldError::invalid_card("Rank must be between 1 and 13 but not 2", card));
        }
        Ok(Meld {
            rank: card.rank,
            cards: vec![card],
            natural: true,
        })
    }

    //adds card to the meld and returns a reference to the card inside the meld
    //if there is an error resturns the card within the error
    pub(crate) fn add<'a>(&'a mut self, card: Card) -> Result<&'a Card, MeldError> { 
        //if the card is a 3 return an error containing the card
        if card.rank == 3 {
            return Err(MeldError::invalid_card("Cannot add 3 to a meld", card));
        }
        //if the card is a wild check to make sure there are not more wilds than non wilds in the meld
        //if there are more wilds than non wilds, return an error containing the card
        if card.is_wild() {
            let mut wilds = 0;
            for c in &self.cards {
                if c.is_wild() {
                    wilds += 1;
                } 
            }
            if wilds >= self.cards.len() - wilds {
                return Err(MeldError::invalid_card("Too many wilds in meld", card));
            }
            self.natural = false;
        } 
        //if the rank doesnt match the meld, return an error containing the card
        else if card.rank != self.rank {
            return Err(MeldError::invalid_card("Only add cards of the same rank", card));
        }
        //add the card to the meld and return a reference to the card inside the meld
        self.cards.push(card);
        Ok(self.cards.last().unwrap())
    }

    //removes the card at the given index and returns it
    pub(crate) fn remove(&mut self, index: usize) -> Result<Card, MeldError> {
        if index >= self.cards.len() {
            return Err(MeldError::invalid_index("Index out of bounds"));
        }
        Ok(self.cards.remove(index))
    }

    //clears the meld and returns the cards
    pub(crate) fn clear(self) -> Vec<Card> {
        self.cards
    }

    //returns a reference to the cards in the meld 
    pub(crate) fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub(crate) fn combine(&mut self, meld: Meld) -> Result<(), MeldError> {
        if self.rank != meld.rank {
            return Err(MeldError::invalid_rank("Cannot combine melds of different ranks"));
        }
        self.cards.extend(meld.cards);
        Ok(())
    }

}



