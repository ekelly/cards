#![allow(dead_code)]

extern crate rand;

mod card;
mod deck;
mod hand;

#[cfg(test)]
mod tests {

    use deck::Deck;
    use card::Rank;

    #[test]
    fn create_shuffled_deck() {
        let d = Deck::new_shuffled();
        assert_eq!(52, d.len());
        assert_ne!(Deck::new_unshuffled(), d);
    }

    #[test]
    fn create_unshuffled_deck() {
        let d = Deck::new_unshuffled();
        assert_eq!(52, d.len());
    }

    #[test]
    fn create_shuffled_subset_deck() {
        let mut d = Deck::new_shuffled_subset(Rank::Six, Rank::King);
        assert_eq!(32, d.len());
        for _ in 0..32 {
            let c = d.deal();
            assert!(c.unwrap().rank.to_n() >= 6);
            assert!(c.unwrap().rank.to_n() <= 13);
        }
        assert_ne!(Deck::new_unshuffled_subset(Rank::Six, Rank::King), d);
    }
}
