extern crate cards;

use cards::deck::{Deck};
use cards::card::{Card, Suit, Value};

#[test]
fn draw_a_card() {
    let mut deck = Deck::new_shuffled();
    let _card = deck.draw().ok().unwrap();
    //println!("The card is: {}", _card);
}

#[test]
fn draw_a_few_cards() {
    let mut deck = Deck::new_shuffled();
    let _cards = deck.draw_n(&5).ok().unwrap();
    //println!("The cards are: {:?}", _cards.iter().map(|x| x.short_string()).collect::<Vec<String>>());

    //use std::str::{StrVector};
    //http://doc.rust-lang.org/0.12.0/std/str/trait.StrVector.html
    //println!("The cards are: {}", cards.iter().map(|x| x.short_string()).collect::<StrVector>().connect(" "));
}

#[test]
fn draw_all_cards() {
    let mut deck = Deck::new_unshuffled();
    deck.draw_n(&52).ok().unwrap();
}

// translates a card to a value between 0 and 51 inclusive
fn card_to_value(card: Card) -> usize {
    let Card(ref card_value, ref card_suit) = card;

    let value = 4*match card_value {
        &Value::Two => 0,
        &Value::Three => 1,
        &Value::Four => 2,
        &Value::Five => 3,
        &Value::Six => 4,
        &Value::Seven => 5,
        &Value::Eight => 6,
        &Value::Nine => 7,
        &Value::Ten => 8,
        &Value::Jack => 9,
        &Value::Queen => 10,
        &Value::King => 11,
        &Value::Ace => 12,
    };
    let suit = match card_suit {
        &Suit::Hearts => 0,
        &Suit::Spades => 1,
        &Suit::Diamonds => 2,
        &Suit::Clubs => 3,
    };

    suit + value
}

#[test]
fn draw_all_cards_and_check() {
    let mut deck = Deck::new_unshuffled();

    let mut cards = [false; 52];

    for _ in 0..52 {
        let card = deck.draw().ok().unwrap();
        let v = card_to_value(card);
        cards[v] = true;
    }

    for i in 0..52 {
        if !cards[i] {
            panic!("Card not dealt!");
        }
    }
}


#[test]
#[should_panic]
fn draw_too_many_cards() {
    let mut deck = Deck::new_shuffled();
    deck.draw_n(&53).ok().unwrap();
}

#[test]
fn reset_deck() {
    let mut deck = Deck::new_unshuffled();
    deck.draw_n(&52).ok().unwrap();
    deck.reset_unshuffled();
    deck.draw_n(&52).ok().unwrap();
}
