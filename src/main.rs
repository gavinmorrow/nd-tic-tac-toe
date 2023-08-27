use std::collections::VecDeque;

use clap::Parser;
use itertools::Itertools;
use nd_tic_tac_toe::{Game, Piece};

fn main() {
    let args = Cli::parse();
    let mut game = Game::new(args.dim, args.players);

    loop {
        // Print the board
        println!("{}", game.display(args.hide_padding));

        // Get the next player's move
        let player = game.current_player();
        println!("{}:", player);
        let Ok(coords) = get_player_input() else {
            println!("Invalid input");
            continue;
        };

        let coords = if args.dim % 2 != 0 {
            map_player_input(coords)
        } else {
            coords
        };

        // Check if the player's move is valid
        match game.place_piece(Piece::new(player), coords) {
            Ok(_) => {
                // Check if the game is over
                if game.check_win(player) {
                    // Print the board
                    println!("{}", game.display(args.hide_padding));
                    println!("{} wins!", player);
                    break;
                }
            }
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };
    }
}

fn get_player_input() -> std::io::Result<VecDeque<usize>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.trim().split_whitespace().map(|x| x.parse());

    if !input.clone().all(|r| r.is_ok()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid input",
        ));
    }

    let input: VecDeque<usize> = input.map(|r| r.unwrap()).collect();

    return Ok(input);
}

fn map_player_input(input: VecDeque<usize>) -> VecDeque<usize> {
    let chunks = input
        .into_iter()
        // Turns `x1 y2 x2` into `x2 y2 x1`
        .rev()
        // Then `(x2 y2) (x1)`
        .chunks(2);

    let mut chunks_vec = Vec::new();
    for chunk in &chunks {
        chunks_vec.push(chunk);
    }

    chunks_vec
        .into_iter()
        // Then `(x1) (x2 y2)`
        .rev()
        // Then `x1 x2 y2`
        .flatten()
        .collect()
}

/// Start an n-dimensional tic-tac-toe game.
#[derive(Parser)]
struct Cli {
    /// The number of dimensions in the game
    #[arg(short = 'd', long = "dim")]
    dim: usize,

    /// The number of players in the game
    #[arg(short = 'p', long = "players")]
    players: u32,

    // Whether or not to pad each piece with spaces
    #[arg(short = 's', long = "hide-padding")]
    hide_padding: bool,
}
