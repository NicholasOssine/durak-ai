mod cards;
mod endgame;
mod engine;
mod hand;
mod protocol;
mod rollout;
mod search;

use rand::rngs::SmallRng;
use std::io::{self, BufRead, Write};

fn main() {
    let mut rng = rand::make_rng::<SmallRng>();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        if line.trim() == "quit" {
            break;
        }

        writeln!(stdout, "{}", protocol::respond(&line, &mut rng)).unwrap();
        stdout.flush().unwrap();
    }
}
