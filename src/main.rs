// use rocket::prelude::*

pub mod Core;

use std::vec;

use crate::Core::bot;
use crate::Core::game;

fn main() {
    // println!("{:?} wins!",run_cli(true, bot::RandomMove));
    println!("{}",game::Game::new(2).render_cli(
            Some(vec!["0","1","2"]
            .iter()
            .map(|x| x.to_string())
            .collect()),
        None).unwrap());
}

fn run_cli(is_user_x: bool, curr_bot: impl bot::Bot) -> Result<game::Cell, String> {
    use std::io;

    fn get_move(splitter: Option<char>) -> Result<Vec<usize>, &'static str> {
        //output [0;8]
        let splitter = splitter.unwrap_or(' ');

        let mut i: String = String::new();
        if io::stdin().read_line(&mut i).is_err() {
            return Err("Bad input");
        }

        // let ans:Vec<u8>;
        // for elem in i.split(splitter).collect(){
        //     ans.push(elem.trim().parse::<u8>())
        // };
        // Ok(ans)

        i.split(splitter)
            .map(|s| s.trim().parse::<usize>().map_err(|_| "Failed to parse"))
            .collect()
    }

    let mut g = game::Game::new(2);
    if !is_user_x {
        g.place_mark(curr_bot.make_move(&g), false, false);
    }
    'single_game: loop {
        println!("{}\nMove: ", &g.render_cli(None, None).unwrap());
        if let Some(r) = g.place_mark(get_move(None)?, false, true)? {
            if r != game::Cell::Empty {
                return Ok(r);
            }
        };
        if let Some(r) = g.place_mark(curr_bot.make_move(&g), false, true)? {
            if r != game::Cell::Empty {
                return Ok(r);
            }
        };
    }
}
