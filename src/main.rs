extern crate discord;
extern crate regex;
extern crate serde_json;

mod audio;

use std::env;
use discord::{Discord, State};
use discord::model::Event;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 || args[1] == "--help" {
        println!("syntax: {} token", args[0]);
        return
    }
    let token = &args[1];
    println!("token = {}", token);

    let discord = Discord::from_bot_token(token).expect("Expected token");
    let (mut connection, ready) = discord.connect().expect("connect failed");
    println!("[Ready] {} is serving {} servers", ready.user.username, ready.servers.len());
    let mut state = State::new(ready);
    connection.sync_calls(&state.all_private_channels());
    let corrections: HashMap<_, _> = [
            ("frace", "frase*"), ("fraces", "frases*"),
            ("valla", "\\~\\~<:fence:266639249779064833>\\~\\~"),
            ("vallas", "\\~\\~<:fence:266639249779064833> <:fence:266639249779064833>\\~\\~"),
            ].iter().cloned().collect();
    let re_correction = Regex::new(&format!(r"\b({})\b", corrections.keys().map(|s|*s).collect::<Vec<&str>>().join("|"))).unwrap();

    loop {
        let event = match connection.recv_event() {
            Ok(event) => event,
            Err(err) => {
                println!("[Warning] Receive error: {:?}", err);
                if let discord::Error::WebSocket(..) = err {
                    // Handle the websocket connection being dropped
                    let (new_connection, ready) = discord.connect().expect("connect failed");
                    connection = new_connection;
                    state = State::new(ready);
                    println!("[Ready] Reconnected successfully.");
                }
                if let discord::Error::Closed(..) = err {
                    break
                }
                continue
            },
        };
        state.update(&event);

        match event {
            Event::MessageCreate(message) => {
                let chan = &message.channel_id;
                let reply = |text: &str| {
                    let res = discord.send_message(chan, text, "", false);
                    match res {
                        Ok(_) => {},
                        Err(err) => println!("[Warning] {:?}", err)
                    }
                };

                println!("Llegó el mensaje con content: {}", message.content);
                match re_correction.find(&message.content).and_then(|m| corrections.get(m.as_str())) {
                    Some(s) => reply(&(message.author.name + ": " + s)),
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
