use std::collections::VecDeque;

use clap::Parser;
use itertools::Itertools;
use nd_tic_tac_toe::{Game, Piece};

fn main() {
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    let args = Cli::parse();
    let mut game = Game::new(args.dim, args.players);

    let mut top_message: String = format!(
        "Starting a {}-dimensional tic-tac-toe game with {} players",
        args.dim, args.players
    );
    let mut last_error: Option<String> = None;
    loop {
        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        // Print the top message
        println!("{}", top_message);

        // Print the board
        println!("{}\n", game.display(args.hide_padding));

        // Get the next player's move
        let player = game.current_player();
        println!(
            "{}: \x1b[1m{}\x1b[0m",
            player,
            if let Some(last_error) = last_error {
                last_error
            } else {
                "".to_string()
            }
        );
        let Ok(coords) = get_player_input() else {
            last_error = Some("Invalid input".to_string());
            continue;
        };

        top_message = format!("Last move: {} at {:?}", player, coords);

        // Adjust input
        let coords = if args.dim % 2 != 0 {
            map_player_input(coords)
        } else {
            coords
        };

        // Check if the player's move is valid
        match game.place_piece(Piece::new(player), coords) {
            Ok(_) => {
                last_error = None;

                // Check if the game is over
                if game.check_win(player) {
                    // Clear the screen
                    print!("\x1B[2J\x1B[1;1H");

                    // Print the board
                    println!("{}", game.display(args.hide_padding));
                    println!("\x1b[1m{}\x1b[1m wins!\x1b[0m", player);

                    // Exit
                    break;
                }
            }
            Err(e) => {
                last_error = Some(e.to_string());
                continue;
            }
        };
    }

    println!("Game over. Goodbye!")
}

fn get_player_input() -> std::io::Result<VecDeque<usize>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.split_whitespace().map(|x| x.parse());

    if !input.clone().all(|r| r.is_ok()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid input",
        ));
    }

    let input: VecDeque<usize> = input.map(|r| r.unwrap()).collect();
    Ok(input)
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
