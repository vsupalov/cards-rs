//use std::fmt;
use rand;
use rand::{Rng};

use card::{Suit, Value, Card};

pub struct Deck {
    count_dealt: usize,
    cards: [u16; 52],
}

impl Deck {
    pub fn new() -> Deck {
        let mut d = Deck {
            count_dealt: 0,
            cards: [0; 52],
        };

        let mut value = 0;
        for x in d.cards.iter_mut() {
            *x = value;
            value += 1;
        }

        d.shuffle();

        d
    }

    pub fn reset(&mut self) {
        self.count_dealt = 0;
        self.shuffle();
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut self.cards);
    }

    fn create_card_for_value(&self, value: &u16) -> Card {
        let suit = match value/13 {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            _ => panic!("Unexpected suit conversion number")
        };

        let value = match value%13 {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::Queen,
            11 => Value::King,
            12 => Value::Ace,
            _ => panic!("Unexpected value conversion number")
        };

        Card(value, suit)
    }

    pub fn draw(&mut self) -> Card {
        let value = &self.cards[self.count_dealt];
        self.count_dealt+=1;

        self.create_card_for_value(value)
    }

    pub fn draw_n(&mut self, n: usize) -> Vec<Card> {
        let mut cards = Vec::new();

        for _ in 0..n {
            cards.push(self.draw())
        }
        cards
    }
}

