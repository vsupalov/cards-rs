extern crate rand;

pub mod card;
pub mod deck;

// TODO: this, or something like it might make sense somewhere
/*
use card::{Card};

pub fn card_vec_to_string(cards: Vec<Card>) -> String {
    let mut s = String::new();
    s.push_str("[ ");
    for card in &cards {
        s.push_str(card.short_string().as_str());
        s.push_str(" ");
    }
    s.push_str("]");
    s
}
*/
