use card::{Card, Rank, Suit};
use std::fmt::{Formatter, Display, Error};
use rand;
use rand::Rng;

#[derive(PartialEq,Clone,Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

pub enum DeckError {
    NotEnoughCards
}

impl Deck {

    pub fn new_unshuffled() -> Deck {
        Deck::new_unshuffled_subset(Rank::Two, Rank::Ace)
    }

    pub fn new_unshuffled_subset(rank_min: Rank, rank_max: Rank) -> Deck {
        let mut cards: Vec<Card> = vec![];
        for suit_num in 0..4 {
            let s: Suit = Suit::from_n(suit_num).unwrap();
            for n in rank_min.to_n()..rank_max.to_n()+1 {
                let card = Card { suit: s, rank: Rank::from_n(n).unwrap() };
                cards.push(card);
            }
        }
        Deck { cards: cards }
    }

    pub fn new_shuffled() -> Deck {
        let mut d = Deck::new_unshuffled();
        d.shuffle();
        d
    }

    pub fn new_shuffled_subset(rank_min: Rank, rank_max: Rank) -> Deck {
        let mut d = Deck::new_unshuffled_subset(rank_min, rank_max);
        d.shuffle();
        d
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn deal_n(&mut self, n: usize) -> Result<Vec<Card>, DeckError> {
        if self.cards.len() < n {
            Err(DeckError::NotEnoughCards)
        } else {
            let mut cards: Vec<Card> = Vec::new();
            for _ in 0..n {
                cards.push(self.deal().unwrap());
            }
            Ok(cards)
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let cards: &mut [Card] = self.cards.as_mut_slice();
        rng.shuffle(cards);
    }
}

impl Display for Deck {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[").unwrap();
        for (index, card) in self.cards.iter().enumerate() {
            write!(f, "{}{}", card, if index == self.cards.len() - 1 { "" } else { ", " }).unwrap();
        }
        write!(f, "]")
    }
}