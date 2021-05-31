use std::fs;
use minidom::Element;
use rota::{Card, Arcana, Suit};

const CARDS_NS: &str = "http://data.totl.net/cards/";
const FOAF_NS: &str = "http://xmlns.com/foaf/0.1/";
const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

fn main() {
    let path = String::from("../assets/tarot-rwcs.xml");
    let card = parse_doc(path);

    println!("{:#?}", card);
}

fn parse_doc(path: String) -> Vec<Card> {
    let xml = fs::read_to_string(path)
        .expect("Failed to read file");

    let root: Element = xml.parse().unwrap();

    let deck = match root.get_child("Deck", CARDS_NS) {
        Some(deck) => deck,
        None => panic!("No Deck element found"),
    };

    let mut cards: Vec<Card> = Vec::new();

    for has_card in deck.children() {
        if !has_card.is("hasCard", CARDS_NS) { continue }

        let node = match has_card.get_child("Card", CARDS_NS) {
            Some(node) => node,
            None => continue,
        };

        let card = Card {
            name: get_name(node),
            number: get_number(node),
            arcana: get_arcana(node),
            suit: get_suit(node),
        };
        cards.push(card);
    }

    cards
}

fn get_name(node: &Element) -> String {
    return match node.get_child("name", FOAF_NS) {
        Some(name) => name.text(),
        None => String::from("None"),
    };
}

fn get_number(node: &Element) -> u8 {
    return match node.get_child("numericValue", CARDS_NS) {
        Some(number) => number.text().parse().unwrap(),
        None => 0,
    };
}

fn get_arcana(node: &Element) -> Arcana {
    for card_type in node.children() {
        if !card_type.is("type", RDF_NS) { continue }

        for (_, value) in card_type.attrs() {
            if value == "http://data.totl.net/tarot/MinorArcanaCard" {
                return Arcana::Minor;
            } else if value == "http://data.totl.net/tarot/MajorArcanaCard" {
                return Arcana::Major;
            } else {
                continue;
            }
        }
    }

    Arcana::Minor
}

fn get_suit(node: &Element) -> Option<Suit> {
    if !node.has_child("suit", CARDS_NS) { return None }

    let suit = node.get_child("suit", CARDS_NS).unwrap();
    let suit = suit.attr("rdf:resource").unwrap();
    let suit: Vec<&str> = suit.rsplit("/").collect();
    let suit = suit[0];

    let suit = if suit == "Swords" {
        Suit::Swords
    } else if suit == "Cups" {
        Suit::Cups
    } else if suit == "Pentacles" {
        Suit::Pentacles
    } else {
        Suit::Wands
    };

    Some(suit)
}
