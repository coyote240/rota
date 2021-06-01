use rota;

fn main() {
    let deck = rota::Deck::new();
    let hand = deck.draw(3);
    println!("{:#?}", hand);
}
