use std::io::Write;
use std::process::{Command, Stdio};
use serde::Serialize;

#[derive(Debug)]
struct Card {
    val: i8,
    quant: i8,
}

struct Msg {
    content: String,
    val: i8,
}
fn main() {
    let mut players = vec![];
    let mut player1 = Command::new("./worker")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    players.push(player1);


    let mut deck: Vec<Card> = Vec::new();
    
    for i in 2..=10 {
        deck.push(Card {val: i, quant: 4,})
    }
    deck.push(Card {val: 10, quant: 16});

    println!("{:?}",deck);

    while !deck.is_empty() {
        let card = rand::random_range(0..deck.len());
        if deck[card].quant > 1 {
            deck[card].quant -= 1;
        }
        else {
            deck.remove(card);
        }
        println!("{:?}", deck);
    }
}
