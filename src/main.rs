use rota;

fn main() {
    let card = rota::Card {
        number: 0,
        name: String::from("The Fool"),
        arcana: rota::Arcana::Major,
        suit: None
    };

    println!("{}", card.arcana);
}
