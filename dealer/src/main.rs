use std::io::{Stdout, Write};
use std::process::{ChildStdout, Command, Stdio};
use serde::Serialize;
use serde_json::to_string;
use std::process::ChildStdin;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
struct Card {
    val: i8,
    quant: i8,
}
#[derive(serde::Serialize)]
struct MsgInit {
    action: String,
    pcard: i8,
    dcard: i8,
}
#[derive(serde::Serialize)]
struct Deal {
    action: String,
    card: i8,
}
#[derive(serde::Deserialize)]
struct BotResponse {
    action: String,
}
fn init(plcard:i8, decard:i8, mut input: &ChildStdin) {
    let initialiser: MsgInit = MsgInit { action: ("init").to_string(), pcard: (plcard), dcard: (decard) };
    let json = serde_json::to_string(&initialiser).unwrap();
    writeln!(input, "{}", json).unwrap();
    input.flush().unwrap();
}

fn deal(card: i8, mut input: &ChildStdin) {
    let deal: Deal = Deal {action: ("deal").to_string(), card: (card)};
    let json = serde_json::to_string(&deal).unwrap();
    writeln!(input, "{}",json).unwrap();
}

fn res() {}

fn main() {
    let mut players = vec![];
    let mut player1 = Command::new("/home/asklipios/bjack/bot1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    players.push(&player1);
    let mut input = player1.stdin.take().unwrap();
    let mut output = player1.stdout.take().unwrap();

    let mut deck: Vec<Card> = Vec::new();
    let mut reader = BufReader::new(output);
    for i in 1..=10 {
        deck.push(Card {val: i, quant: 4,})
    }
    deck.push(Card {val: 10, quant: 16});

    println!("{:?}",deck);

    while !deck.is_empty() {                              
        let mut bot_total = 0;
        let pcardindx = rand::random_range(0..deck.len());
        let pcard = deck[pcardindx].val;
        if deck[pcardindx].quant > 1 {
            deck[pcardindx].quant -= 1;
        }

        else {
            deck.remove(pcardindx);
        }

        let dcardindx = rand::random_range(0..deck.len());
        let dcard = deck[dcardindx].val;
        if deck[dcardindx].quant > 1 {
            deck[dcardindx].quant -= 1;
        }

        else {
            deck.remove(dcardindx);
        }
        
        println!("{:?}", deck);
        bot_total += pcard;
        init(pcard, dcard, &input);

        loop{
            let mut response = String::new();
            reader.read_line(&mut response).unwrap();
            let bot_response: BotResponse = serde_json::from_str(&response.trim()).unwrap();
            let botresponse: &str = &bot_response.action; 
            match botresponse {
                "hit" => {
                    let pcardindx = rand::random_range(0..deck.len());
                    let pcard = deck[pcardindx].val;
                    if deck[pcardindx].quant > 1 {
                        deck[pcardindx].quant -= 1;
                    }

                    else {
                        deck.remove(pcardindx);
                    }
                    deal(pcard, &input);
                    bot_total +=pcard;
                },
                "stand" => {
                    break;
                },
                _ => {
                    eprintln!("Unknown action!");
                    break;
                }
            }
        }
    }
}
