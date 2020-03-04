use std::io;
use rs_poker::core::Hand;
use rs_poker::holdem::MonteCarloGame;

fn main() {
    // let deck = rs_poker::Deck::new();
    // let hand = core::Hand::new_from_str("AdKh").expect("error....");
    // for card in hand.iter() {
        // println!("{}", card);
    // }

    println!("Enter two hands on separate lines:");
    
    let mut hand1 = String::new();
    io::stdin().read_line(&mut hand1).expect("failed read");
    let hand1 = hand1.trim();

    let mut hand2 = String::new();
    io::stdin().read_line(&mut hand2).expect("failed read");
    let hand2 = hand2.trim();

    println!("calculating");

    let hands = [hand1, hand2]
        .iter()
        .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
        .collect();
    let mut g = MonteCarloGame::new_with_hands(hands).expect("Should be able to create a game.");
    let mut wins: [u64; 2] = [0, 0];
    for _ in 0..2_000 {
        let r = g.simulate().expect("There should be one best rank.");
        g.reset();
        wins[r.0] += 1
    }
    println!("Wins = {:?}", wins);
}
