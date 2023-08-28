use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Player(char);

impl Player {
    pub fn new(symbol: char) -> Self {
        Self(symbol)
    }

    pub fn symbol(&self) -> char {
        self.0
    }

    pub fn with_color(&self) -> String {
        let color = match self.0 {
            'X' => "\x1b[1;94m",
            'O' => "\x1b[1;93m",
            'F' => "\x1b[1;95m",
            '•' => "\x1b[2;90m",
            _ => "\x1b[1;1m",
        };
        format!("{}{}\x1b[0m", color, self.0)
    }
}

impl TryFrom<u32> for Player {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self('X')),
            1 => Ok(Self('O')),
            2 => Ok(Self('F')),
            _ => Err(()),
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Player {}", self.symbol()).as_str())
    }
}
