use rand;
use rand::Rng;

use card::Card;

pub struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug)]
pub enum DeckError {
    NotEnoughCards,
    CardNotInDeck,
}

/// A deck can be dealt from and shuffled.
impl Deck {
    //TODO: a deck containing multiple sets of cards? When 52*3 is needed.

    /// Returns a deck where all cards are sorted by Suit, then by Value.
    pub fn new_unshuffled() -> Deck {
        let mut d = Deck { cards: Vec::new() };

        for value in 0..52 {
            d.cards.push(Card::from_value(value).ok().unwrap());
        }
        d
    }

    /// A freshly shuffled deck of 52 cards.
    pub fn new_shuffled() -> Deck {
        let mut d = Deck::new_unshuffled();
        d.shuffle();
        d
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.cards);
    }

    /// An attempt to get a card from the deck. There might not be enough.
    pub fn draw(&mut self) -> Result<Card, DeckError> {
        match self.cards.pop() {
            Some(c) => Ok(c),
            None => Err(DeckError::NotEnoughCards),
        }
    }

    /// An attempt to get n cards from the deck wrapped in a Vec. There might not be enough.
    pub fn draw_n(&mut self, n: usize) -> Result<Vec<Card>, DeckError> {
        if self.cards.len() >= n {
            let mut cards = Vec::new();
            // For as many cards as are requested
            for _ in 0..n {
                // Push a new card
                cards.push(
                    // We get the card to push from self.draw()
                    match self.draw() {
                        Ok(c) => c,
                        Err(_) => return Err(DeckError::NotEnoughCards),
                    },
                );
            }
            Ok(cards)
        } else {
            Err(DeckError::NotEnoughCards)
        }
    }

    pub fn remove(&mut self, card: Card) -> Result<Card, DeckError> {
        // TODO: Use self.cards.remove_item when the API stablizes
        // match self.cards.remove_item(c) {
        //     Some(c) => Ok(c),
        //     None => Err(DeckError::CardNotInDeck)
        // }
        match self.cards.iter().position(|c| *c == card) {
            Some(index) => Ok(self.cards.remove(index)),
            None => Err(DeckError::CardNotInDeck),
        }
    }
}
