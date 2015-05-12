extern crate cards;

use cards::deck::{Deck};

#[test]
fn draw_a_card() {
    let mut deck = Deck::new();
    let _card = deck.draw();
    //println!("The card is: {}", _card);
}

#[test]
fn draw_a_few_card() {
    let mut deck = Deck::new();
    let _cards = deck.draw_n(&5);
    //println!("The cards are: {:?}", _cards.iter().map(|x| x.short_string()).collect::<Vec<String>>());

    //use std::str::{StrVector};
    //http://doc.rust-lang.org/0.12.0/std/str/trait.StrVector.html
    //println!("The cards are: {}", cards.iter().map(|x| x.short_string()).collect::<StrVector>().connect(" "));
}

#[test]
fn draw_all_cards() {
    let mut deck = Deck::new();
    deck.draw_n(&52);
}

#[test]
#[should_panic]
fn draw_too_many_cards() {
    let mut deck = Deck::new();
    deck.draw_n(&53);
}

#[test]
fn reset_deck() {
    let mut deck = Deck::new();
    deck.draw_n(&52);
    deck.reset();
    deck.draw_n(&52);
}
