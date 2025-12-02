use std::io::{Stdout, Write};
use std::process::{ChildStdout, Command, Stdio};
use serde::Serialize;
use serde_json::to_string;
use std::process::ChildStdin;

#[derive(Debug)]
struct Card {
    val: i8,
    quant: i8,
}
#[derive(serde::Serialize)]
struct MsgInit {
    content: String,
    pcard: i8,
    dcard: i8,
}

fn init(plcard:i8, decard:i8, mut input: &ChildStdin) {
    let initialiser: MsgInit = MsgInit { content: ("init").to_string(), pcard: (plcard), dcard: (decard) };
    let json = serde_json::to_string(&initialiser).unwrap();
    writeln!(input, "{}", json).unwrap();
    input.flush().unwrap();
}

fn deal() {}

fn res() {}

fn main() {
    let mut players = vec![];
    let mut player1 = Command::new("/home/asklipios/bjack/bot1")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    players.push(&player1);
    let mut input = player1.stdin.take().unwrap();


    let mut deck: Vec<Card> = Vec::new();
    
    for i in 1..=10 {
        deck.push(Card {val: i, quant: 4,})
    }
    deck.push(Card {val: 10, quant: 16});

    println!("{:?}",deck);

    while !deck.is_empty() {
        let pcard = rand::random_range(0..deck.len());

        if deck[pcard].quant > 1 {
            deck[pcard].quant -= 1;
        }

        else {
            deck.remove(pcard);
        }

        let dcard = rand::random_range(0..deck.len());
        
        if deck[dcard].quant > 1 {
            deck[dcard].quant -= 1;
        }

        else {
            deck.remove(dcard);
        }
        
        println!("{:?}", deck);
        
        init(deck[pcard].val, deck[dcard].val, &input);
    }
}
