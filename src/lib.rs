use std::fmt;
use std::iter::Iterator;
use serde_json;
use serde::{Deserialize, Serialize};
use rand::seq::SliceRandom;


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let db = include_str!("../assets/tarot-rwcs.json");
        return serde_json::from_str(db).unwrap();
    }

    pub fn draw(&self, count: usize) -> Vec<&Card> {
        let drawn: Vec<&Card> = self.cards.choose_multiple(
            &mut rand::thread_rng(), count).collect();

        return drawn;
    }
}

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Card {
    pub number: u8,
    pub name: String,
    pub arcana: Arcana,
    pub suit: Option<Suit>,
    pub depiction: String
}

#[derive(Debug, Copy, Clone)]
#[derive(Deserialize, Serialize)]
pub enum Arcana {
    Minor,
    Major,
}

impl fmt::Display for Arcana {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arcana::Major => write!(f, "Major"),
            Arcana::Minor => write!(f, "Minor"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[derive(Deserialize, Serialize)]
pub enum Suit {
    Swords,
    Cups,
    Wands,
    Pentacles,
}
