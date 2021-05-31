#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Copy, Clone)]
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
pub enum Suit {
    Swords,
    Cups,
    Wands,
    Pentacles,
}

#[derive(Debug)]
pub struct Card {
    pub number: u8,
    pub name: String,
    pub arcana: Arcana,
    pub suit: Option<Suit>,
}

pub struct Deck {
    cards: Vec<Card>,
}

