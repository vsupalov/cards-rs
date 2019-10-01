use std::fmt;

pub enum CardError {
    SuitDoesNotExist,
    ValueDoesNotExist,
    CardDoesNotExist,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn short_string(&self) -> &'static str {
        match *self {
            Suit::Spades => "s",
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Clubs => "c",
        }
    }

    fn from_value(value: u8) -> Result<Suit, CardError> {
        match value / 13 {
            0 => Ok(Suit::Spades),
            1 => Ok(Suit::Hearts),
            2 => Ok(Suit::Diamonds),
            3 => Ok(Suit::Clubs),
            _ => Err(CardError::SuitDoesNotExist),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
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
    fn short_string(&self) -> &'static str {
        match *self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "T",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        }
    }

    fn from_value(value: u8) -> Result<Value, CardError> {
        match value % 13 {
            0 => Ok(Value::Two),
            1 => Ok(Value::Three),
            2 => Ok(Value::Four),
            3 => Ok(Value::Five),
            4 => Ok(Value::Six),
            5 => Ok(Value::Seven),
            6 => Ok(Value::Eight),
            7 => Ok(Value::Nine),
            8 => Ok(Value::Ten),
            9 => Ok(Value::Jack),
            10 => Ok(Value::Queen),
            11 => Ok(Value::King),
            12 => Ok(Value::Ace),
            _ => Err(CardError::ValueDoesNotExist),
        }
    }
}

//TODO: debug still relevant? It was used to print a vec of cards.
/// An unnamed tuple with Value and Suit.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Card {
        Card {
            value: value,
            suit: suit,
        }
    }

    pub(crate) fn from_value(v: u8) -> Result<Card, CardError> {
        let value = match Value::from_value(v) {
            Ok(v) => v,
            Err(_) => return Err(CardError::CardDoesNotExist),
        };

        let suit = match Suit::from_value(v) {
            Ok(s) => s,
            Err(_) => return Err(CardError::CardDoesNotExist),
        };

        Ok(Card::new(value, suit))
    }
}

// so cards can be printed using fmt method
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.value.short_string(),
            self.suit.short_string()
        )
    }
}
