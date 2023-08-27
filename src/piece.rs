use std::fmt::Display;

use super::Player;

#[derive(Debug, Clone)]
pub struct Piece {
    pub player: Option<Player>,
}

impl Piece {
    pub fn new(player: Player) -> Self {
        Self {
            player: Some(player),
        }
    }

    pub fn empty() -> Self {
        Self { player: None }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If there isn't a player, create a temporary "player" that will be
        // used only for displaying. (it simplifies the code)
        let player = self.player.unwrap_or_else(|| Player::new('•'));
        let data = &player.with_color();

        f.write_str(data)
    }
}
