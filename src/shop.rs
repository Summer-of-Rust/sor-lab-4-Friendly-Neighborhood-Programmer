use std::{cmp::Ordering, fmt};

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        let mut other_cards = other.cards.iter();

        let mut wins: usize = 0;
        let mut losses: usize = 0;

        for card in &self.cards {
            match card.print_fight(other_cards.next().unwrap()) {
                FightResult::Win => wins += 1,
                FightResult::Loss => losses += 1,
                _ => (),
            }
        }

        match wins.cmp(&losses) {
            Ordering::Greater => FightResult::Win,
            Ordering::Less => FightResult::Loss,
            Ordering::Equal => FightResult::Tie,
        }

        // let mut i: usize = 0;
        // let mut j: usize = 0;

        // let wins: usize = self.cards.iter().filter(|card| {
        //         i += 1;
        //         match card.fight(&other.cards[i-1]) {
        //             FightResult::Win => true,
        //             _ => false,
        //         }}).count();

        // let losses: usize = self.cards.iter().filter(|card| {
        //     j += 1;
        //     match card.fight(&other.cards[j-1]) {
        //         FightResult::Loss => true,
        //         _ => false,
        //     }}).count();

        // if wins > losses {
        //     FightResult::Win
        // } else if losses > wins {
        //     FightResult::Loss
        // } else {
        //     FightResult::Tie
        // }
    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}

// #[test]
// fn test_my_shop_fight() {
//     let shop = Shop {
//         cards: vec![
//                 Card {
//                     price: 10,
//                     health: 10,
//                     damage: 12,
//                 },
//         ]
//     };

//     shop.fight_store(&shop);
// }
