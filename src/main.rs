extern crate serde;
extern crate serde_json;
extern crate curl;
extern crate chrono;
extern crate irc;
extern crate regex;

mod game;
mod schedule;
mod command;

use irc::client::prelude::*;
use irc::client::data::command as irc_command;
use std::time::Duration;
use std::thread;
use regex::Regex;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    let config = server.config();
    let channel = config.channels()[0];
    server.identify().unwrap();
    // Parses: `<botname><?char> <command>` or `<botname><?char> <command> <?args...>`.
    let pattern = format!(r"^{}.? (\w+)(?:\s([\w]+(?:\s[\w]+)*))?", config.nickname());
    let valid_commands = Regex::new(&pattern).unwrap();
    for message in server.iter() {
        let message = message.unwrap();
        // We can't process commands from a PRIVMSG if there's no source nick.
        let nick = match message.source_nickname() {
            Some(nick) => nick,
            None => continue
        };
        if let irc_command::Command::PRIVMSG(_, ref command) = message.command {
            let captures = valid_commands.captures(command);
            // If the message didn't match the regular expression, forget about it.
            if captures.is_none() {
                continue;
            }

            let captures = captures.unwrap();
            let command = captures.at(1).unwrap();
            if config.is_owner(nick) && command == "die" {
                break;
            }

            let args = match captures.at(2) {
                Some(args) => args,
                None => ""
            };
            let responses = command::execute(command, args.split(' ').collect());
            if let Ok(responses) = responses {
                for response in responses.iter() {
                    server.send_privmsg(channel, response).unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            } else if let Err(error_msg) = responses {
                server.send_privmsg(channel, &error_msg).unwrap();
            }
        }
    }
}
