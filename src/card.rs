use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn short_string(&self) -> &str {
        match self {
            &Suit::Spades => "s",
            &Suit::Hearts => "h",
            &Suit::Diamonds => "d",
            &Suit::Clubs => "c",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    // no jokers
}

impl Value {
    fn short_string(&self) -> &str {
        match self {
            &Value::Two => "2",
            &Value::Three => "3",
            &Value::Four => "4",
            &Value::Five => "5",
            &Value::Six => "6",
            &Value::Seven => "7",
            &Value::Eight => "8",
            &Value::Nine => "9",
            &Value::Ten => "T",
            &Value::Jack => "J",
            &Value::Queen => "Q",
            &Value::King => "K",
            &Value::Ace => "A",
        }
    }
}

//TODO: debug still relevant? It was used to print a vec of cards.
/// An unnamed tuple with Value and Suit.
#[derive(Debug, PartialEq)]
pub struct Card(pub Value, pub Suit);

impl Card {
    /// Returns "As" for an ace of spades, a notation popular in poker.
    pub fn short_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.0.short_string());
        s.push_str(self.1.short_string());
        s
    }
}

// so cards can be printed using fmt method
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.short_string())
    }
}
