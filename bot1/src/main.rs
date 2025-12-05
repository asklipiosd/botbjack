use std::{io::stdin, ptr::null};
use serde::{Serialize,Deserialize};
use serde_json::to_string;

#[derive(Deserialize, Debug)]
struct Msg {
    action: String,
    pcard: Option<i8>,
    dcard: Option<i8>,
}

fn hit(){
    let msg: Msg = Msg { action: ("hit").to_string(), pcard: Option,dcard: Option};
    let json = serde_json::to_string(&msg).unwrap();
    println!("{}",json);
}
fn stand() {   
    let msg: Msg = Msg { action: ("stand").to_string(), pcard: Option,dcard: Option};
    let json = serde_json::to_string(&msg).unwrap();
    println!("{}",json);}
fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut total = 0;
    let mut cards:Vec<i8>;
    let mut ace_count = 0;
    loop {
        let mut line = String::new();
        let msg: Msg = serde_json::from_str(line.trim()).unwrap();

        match handle.read_line(&mut line) {
            Ok(0) => {
                println!("stdin closed, exiting");
                break;
            },
            Ok(n) => {
                println!("Read {} bytes", n);
                //init 
                if msg.action == "init"  {
                    total = 0;
                    ace_count = 0;
                    for i in 0..cards.len() { cards.pop();}
                        cards.push(msg.pcard.unwrap());
                    for i in 0..cards.len() {
                        if cards[i] == 1 {
                            ace_count += 1;
                        }
                        else {
                            total += msg.pcard.unwrap();
                        }
                    }
                    for i in 0..ace_count {
                        if total < 11 {
                            total += 11;
                        }
                        else {
                            total += 1;
                        }
                    }
                    if total < 17 {
                        hit();
                    }
                    else if total == 17 && ace_count > 0{
                        hit();
                    }
                    else {
                        stand();
                    }
                }
                else {
                    ace_count = 0;
                    for i in 0..cards.len() { cards.pop();}
                        cards.push(msg.pcard.unwrap());
                    for i in 0..cards.len() {
                        if cards[i] == 1 {
                            ace_count += 1;
                        }
                        else {
                            total += msg.pcard.unwrap();
                        }
                    }
                    for i in 0..ace_count {
                        if total < 11 {
                            total += 11;
                        }
                        else {
                            total += 1;
                        }
                    }
                    if total < 17 {
                        hit();
                    }
                    else if total == 17 && ace_count > 0{
                        hit();
                    }
                    else {
                        stand();
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading: {}", e);
                break;
            },
        }
        let msg: Msg = serde_json::from_str(line.trim()).unwrap();
    }
}
