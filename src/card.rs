use std::str::FromStr;
use std::fmt::{Formatter, Display, Error};

/// One of the four Suits: Heart, Spade, Diamond, Club.
#[derive(PartialEq,Clone,Copy,Debug)]
#[repr(u32)]
pub enum Suit {
    /// The suit of clubs.
    Club,
    /// The suit of diamonds.
    Diamond,
    /// The suit of spades.
    Spade,
    /// The suit of hearts.
    Heart,
}

impl Suit {

    pub fn from_n(n: u32) -> Result<Self, String> {
        match n {
            0 => Ok(Suit::Club),
            1 => Ok(Suit::Diamond),
            2 => Ok(Suit::Spade),
            3 => Ok(Suit::Heart),
            _ => Err(format!("Invalid suit: {}", n)),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Suit::Heart => "♥",
            Suit::Spade => "♠",
            Suit::Diamond => "♦",
            Suit::Club => "♣",
        }.to_string()
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "H" | "h" | "heart" | "Suit::Heart" | "Heart" => Ok(Suit::Heart),
            "C" | "c" | "club" | "Suit::Club" | "Club" => Ok(Suit::Club),
            "S" | "s" | "spade" | "Suit::Spade" | "Spade" => Ok(Suit::Spade),
            "D" | "d" | "diamond" | "Suit::Diamond" | "Diamond" => Ok(Suit::Diamond),
            _ => Err(format!("invalid suit: {}", s)),
        }
    }
}

#[derive(PartialEq,Clone,Copy,Debug)]
#[repr(u32)]
pub enum Rank {
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
    Ace
}

impl Rank {

    pub fn from_n(n: u8) -> Result<Self, String> {
        match n {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            14 => Ok(Rank::Ace),
            _ => Err(format!("Invalid rank: {}", n)),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }.to_string()
    }

    pub fn to_n(&self) -> u8 {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(PartialEq,Clone,Copy,Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Display for Card {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}", self.rank.to_string(), self.suit.to_string())
    }
}