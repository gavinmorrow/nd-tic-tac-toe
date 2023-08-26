mod board;
mod game;
mod piece;
mod player;

use board::Board;
pub use game::Game;
pub use piece::Piece;
use player::Player;

#[derive(Debug)]
pub enum PlacePieceError {
    OutOfBounds,
    Occupied,
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_3d_winner(winner: Player, loser: Player) -> Game {
        let mut game = Game::new(3, 2);
        let p0 = loser;
        let p1 = winner;

        // Insert pieces
        game.place_piece(Piece::new(p0, vec![1, 1, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![2, 2, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![1, 2, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![1, 3, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 1, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 1, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 3, 3])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 0])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 2, 2])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 3, 3])).unwrap();
        game.place_piece(Piece::new(p0, vec![1, 1, 3])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 2, 2])).unwrap();

        game
    }

    fn setup_4d_both_win(p0: Player, p1: Player) -> Game {
        let mut game = Game::new(4, 2);

        {
            // Insert pieces
            // vec![
            //     p0, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, p1,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,

            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, p1, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, p0, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,

            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, p1, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, p0, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,

            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, p1, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, p0, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,

            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    p1, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,
            //     pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, pn,    pn, pn, pn, pn, p0,
            // ];
        }

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0])).unwrap();
        game.place_piece(Piece::new(p1, vec![4, 0, 2, 0])).unwrap();
        game.place_piece(Piece::new(p0, vec![1, 1, 1, 1])).unwrap();
        game.place_piece(Piece::new(p1, vec![3, 0, 2, 1])).unwrap();
        game.place_piece(Piece::new(p0, vec![2, 2, 2, 2])).unwrap();
        game.place_piece(Piece::new(p1, vec![2, 0, 2, 2])).unwrap();
        game.place_piece(Piece::new(p0, vec![3, 3, 3, 3])).unwrap();
        game.place_piece(Piece::new(p1, vec![1, 0, 2, 3])).unwrap();
        game.place_piece(Piece::new(p0, vec![4, 4, 4, 4])).unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 2, 4])).unwrap();

        game
    }

    #[test]
    fn test_3d_p0_win() {
        let p0 = Player::new('X');
        let p1 = Player::new('O');
        let game = setup_3d_winner(p0, p1);

        // Check winner
        assert!(game.check_win(p0));
        assert!(!game.check_win(p1));
    }

    #[test]
    fn test_3d_p1_win() {
        let p0 = Player::new('X');
        let p1 = Player::new('O');
        let game = setup_3d_winner(p1, p0);

        // Check winner
        assert!(!game.check_win(p0));
        assert!(game.check_win(p1));
    }

    #[test]
    fn test_4d_both_win() {
        let p0 = Player::new('X');
        let p1 = Player::new('O');
        let game = setup_4d_both_win(p0, p1);

        // Check winner
        assert!(game.check_win(p0));
        assert!(game.check_win(p1));
    }

    #[test]
    fn test_5d_win() {
        let p0 = Player::new('X');
        let p1 = Player::new('O');
        let mut game = Game::new(5, 2);

        // Insert pieces
        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 0]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 1, 0, 1]))
            .unwrap();

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 1]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 0, 4, 1]))
            .unwrap();

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 2]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 2, 0, 0, 1]))
            .unwrap();

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 3]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![1, 0, 0, 0, 1]))
            .unwrap();

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 4]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 2, 2, 0, 1]))
            .unwrap();

        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 5]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 2, 2, 2, 1]))
            .unwrap();

        assert!(game.check_win(p0));
    }

    #[test]
    #[should_panic]
    fn test_5d_occupied() {
        let mut game = Game::new(5, 2);
        let p0 = Player::new('X');
        let p1 = Player::new('O');

        // Insert pieces
        game.place_piece(Piece::new(p0, vec![0, 0, 0, 0, 0]))
            .unwrap();
        game.place_piece(Piece::new(p1, vec![0, 0, 0, 0, 0]))
            .unwrap();
    }
}
