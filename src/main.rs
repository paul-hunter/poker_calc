use std::io;
use rs_poker::core::Hand;
use rs_poker::core::Rank;
use rs_poker::core::Value;
use rs_poker::core::Flattenable;
use rs_poker::core::FlatDeck;
use rs_poker::core::Deck;
use rs_poker::core::Rankable;
use rs_poker::holdem::MonteCarloGame;
use rs_poker::holdem::RangeParser;

#[derive(Debug)]
struct FlopResult {
    high_card : i32,
    one_pair : i32,
    two_pair : i32,
    three_of_a_kind : i32,
    straight : i32,
    flush : i32,
    full_house : i32,
    four_of_a_kind : i32,
    straight_flush : i32,
}

fn main() {
    // let deck = rs_poker::Deck::new();
    // let hand = core::Hand::new_from_str("AdKh").expect("error....");
    // for card in hand.iter() {
        // println!("{}", card);
    // }

    // let h1 = Hand::new_from_str("AsAh9c8c7c").unwrap();
    // let h2 = Hand::new_from_str("AdAc9c8c7c").unwrap();

    // println!("h1 rank: {:?}", h1.rank());
    // println!("hl rank: {:?}", h2.rank());

    // println!("Enter two hands on separate lines:");
    
    // let mut hand1 = String::new();
    // io::stdin().read_line(&mut hand1).expect("failed read");
    // let hand1 = hand1.trim();

    // let mut hand2 = String::new();
    // io::stdin().read_line(&mut hand2).expect("failed read");
    // let hand2 = hand2.trim();

    // println!("calculating");

    // let hands = [hand1, hand2]
    //     .iter()
    //     .map(|s| Hand::new_from_str(s).expect("Should be able to create a hand."))
    //     .collect();
    // let mut g = MonteCarloGame::new_with_hands(hands).expect("Should be able to create a game.");
    // let mut wins: [u64; 2] = [0, 0];
    // for _ in 0..20 {
    //     let r = g.simulate().expect("There should be one best rank.");
    //     g.reset();
    //     wins[r.0] += 1
    // }
    // println!("Wins = {:?}", wins);

    // println!("enter a range: ");
    
    // let mut range = String::new();
    // io::stdin().read_line(&mut range).expect("failed read");

    // let hands = RangeParser::parse_one(&range).unwrap();

    loop {
        println!("Enter a hand:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed read");
        let trimmed = input.trim();

        let hand = Hand::new_from_str(&trimmed).unwrap();
        let rank_result = find_flopping_odds(&hand).unwrap();

        println!("{:?}", rank_result);
    }
}

fn parse_hands(range : String) {
    let hands = RangeParser::parse_one(&range).unwrap();

    for hand in hands {
        // println!("{}", hand[0].value.to_char());
        println!("{}{}", hand[0], hand[1]);
    }

}


fn find_flopping_odds(hand : &Hand) -> Result<FlopResult, String> {

    if hand.len() != 2 {
        return Err(String::from("Hand passed in doesn't have 2 cards."));
    }

    let mut flop_result = FlopResult {
        high_card: 0,
        one_pair : 0,
        two_pair : 0,
        three_of_a_kind : 0,
        straight : 0,
        flush : 0,
        full_house : 0,
        four_of_a_kind : 0,
        straight_flush : 0,
    };

    for i in 0..10000 {

        let mut deck = Deck::default();
        let mut five_card_hand = Hand::default();

        five_card_hand.push(hand[0]);
        five_card_hand.push(hand[1]);

        deck.remove(hand[0]);
        deck.remove(hand[1]);

        let mut flat_deck = Flattenable::flatten(deck);
        flat_deck.shuffle();
        for i in 0..3 {
            five_card_hand.push(flat_deck.deal().unwrap());
        }

        // for x in five_card_hand.iter() {
        //     println!("{:?}{:?}", x.value, x.suit);
        // }
        let rank = five_card_hand.rank_five();

        match rank {
            Rank::HighCard(_) => flop_result.high_card += 1,
            Rank::OnePair(x) => flop_result.one_pair += 1,
            Rank::TwoPair(x) => flop_result.two_pair += 1,
            Rank::ThreeOfAKind(x) => flop_result.three_of_a_kind += 1,
            Rank::Straight(x) => flop_result.straight += 1,
            Rank::Flush(x) => flop_result.flush += 1,
            Rank::FullHouse(x) => flop_result.full_house += 1,
            Rank::FourOfAKind(x) => flop_result.four_of_a_kind += 1,
            Rank::StraightFlush(x) => flop_result.straight_flush += 1,
        }
    }

    Ok(flop_result)
}