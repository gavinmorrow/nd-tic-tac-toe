use clap::Parser;
use nd_tic_tac_toe::{Game, Piece};

fn main() {
    let args = Cli::parse();
    let mut game = Game::new(args.dim, args.players);

    loop {
        // Print the board
        println!("{}", game);

        // Get the next player's move
        let player = game.current_player();
        println!("{}:", player);
        let Ok(coords) = get_player_input() else {
            println!("Invalid input");
            continue;
        };

        // Check if the player's move is valid
        match game.place_piece(Piece::new(player, coords)) {
            Ok(_) => {
                // Check if the game is over
                if game.check_win(player) {
                    // Print the board
                    println!("{}", game);
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

fn get_player_input() -> std::io::Result<Vec<usize>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.trim().split_whitespace().map(|x| x.parse());

    if !input.clone().all(|r| r.is_ok()) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid input",
        ));
    }

    let input: Vec<usize> = input.map(|r| r.unwrap()).collect();

    let [x1, y1, y2, x2] = input[..] else {
        // println!("{:?}", input);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input"));
    };

    Ok(vec![x1, y1, x2, y2])
}

/// Start an n-dimensional tic-tac-toe game.
#[derive(Parser)]
struct Cli {
    /// The number of dimensions in the game
    // #[arg(short="d",long="dim")]
    dim: usize,
    /// The number of players in the game
    // #[arg(short="p",long="players")]
    players: u32,
}
