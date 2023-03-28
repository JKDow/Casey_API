#[derive(Debug)]
struct Meld {
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

#[derive(Debug)]
struct Player {
    id: usize,
    hand: Vec<Card>,
    melds: Vec<Meld>,
    temp: Vec<Meld>,
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