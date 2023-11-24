mod number;
mod superposition;

pub use number::Number;
use superposition::Superposition;

use owo_colors::{OwoColorize, Stream::Stdout};

use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Square {
    Incomplete(Superposition),
    PlayerMove(Number),
    Starting(Number),
}
impl Square {
    pub fn collapsed_number(&self) -> Option<Number> {
        match self {
            Self::Incomplete(_superposition) => None,
            Self::PlayerMove(collapsed) | Self::Starting(collapsed) => Some(*collapsed),
        }
    }

    pub fn remove(&mut self, number: Number) -> bool {
        match self {
            Self::Incomplete(superposition) => superposition.remove(number),
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => false,
        }
    }

    pub fn superposition_number(&self) -> Option<usize> {
        match self {
            Self::Incomplete(superposition) => Some(superposition.superposition_number()),
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => None,
        }
    }

    pub fn try_move(&mut self, number: Number) -> bool {
        match self {
            Self::Incomplete(superposition) => {
                if superposition.contains(number) {
                    *self = Self::PlayerMove(number);
                    true
                } else {
                    false
                }
            }
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => false,
        }
    }

    pub fn try_random_move(&mut self) -> Option<Number> {
        match self {
            Self::Incomplete(superposition) => {
                let number = superposition.collapse_random()?;

                *self = Self::PlayerMove(number);

                Some(number)
            }
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => None,
        }
    }

    pub fn try_random_set(&mut self) -> Option<Number> {
        match self {
            Self::Incomplete(superposition) => {
                let number = superposition.collapse_random()?;

                *self = Self::Starting(number);

                Some(number)
            }
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => None,
        }
    }

    pub fn try_set(&mut self, number: Number) -> bool {
        match self {
            Self::Incomplete(superposition) => {
                if superposition.contains(number) {
                    *self = Self::Starting(number);
                    true
                } else {
                    false
                }
            }
            Self::PlayerMove(_collapsed) | Self::Starting(_collapsed) => false,
        }
    }

    pub fn try_undo_move(&mut self) -> bool {
        match self {
            Self::Incomplete(_) | Self::Starting(_) => false,
            Self::PlayerMove(_collapsed) => {
                *self = Self::Incomplete(Superposition::default());
                true
            }
        }
    }

    pub fn try_undo_set(&mut self) -> bool {
        match self {
            Self::Incomplete(_) | Self::PlayerMove(_) => false,
            Self::Starting(_collapsed) => {
                *self = Self::Incomplete(Superposition::default());
                true
            }
        }
    }
}
impl Default for Square {
    fn default() -> Self {
        Self::Incomplete(Superposition::default())
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incomplete(displayable) => f.write_str(&format!(
                "{}{}",
                displayable
                    .to_string()
                    .if_supports_color(Stdout, |text| text.bright_yellow()),
                String::new().if_supports_color(Stdout, |text| text.default_color())
            )),
            Self::PlayerMove(displayable) => f.write_str(&format!(
                "{}{}",
                displayable
                    .to_string()
                    .if_supports_color(Stdout, |text| text.bright_green()),
                String::new().if_supports_color(Stdout, |text| text.default_color())
            )),
            Self::Starting(displayable) => f.write_str(&format!(
                "{}{}",
                displayable
                    .to_string()
                    .if_supports_color(Stdout, |text| text.bright_cyan()),
                String::new().if_supports_color(Stdout, |text| text.default_color())
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ansi_str::AnsiStr;
    use pretty_assertions::assert_eq;

    #[test]
    fn square_displays_correctly() {
        let square_displays: Vec<_> = vec![
            Square::Starting(Number::One),
            Square::Starting(Number::Two),
            Square::Starting(Number::Three),
            Square::Starting(Number::Four),
            Square::Starting(Number::Five),
            Square::Starting(Number::Six),
            Square::Starting(Number::Seven),
            Square::Starting(Number::Eight),
            Square::Starting(Number::Nine),
            Square::PlayerMove(Number::One),
            Square::PlayerMove(Number::Two),
            Square::PlayerMove(Number::Three),
            Square::PlayerMove(Number::Four),
            Square::PlayerMove(Number::Five),
            Square::PlayerMove(Number::Six),
            Square::PlayerMove(Number::Seven),
            Square::PlayerMove(Number::Eight),
            Square::PlayerMove(Number::Nine),
            Square::default(),
            {
                // Square with only one superposition option
                let mut superposition = Superposition::default();
                for number in &Number::ALL[0..8] {
                    superposition.remove(*number);
                }
                Square::Incomplete(superposition)
            },
            {
                // Square with no possible options
                let mut superposition = Superposition::default();
                for number in Number::ALL {
                    superposition.remove(number);
                }
                Square::Incomplete(superposition)
            },
        ]
        .into_iter()
        .map(|square| square.to_string().ansi_strip().to_string())
        .collect();

        let correct_displays = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "1", "2", "3", "4", "5", "6", "7", "8",
            "9", "?", "!", "0",
        ];

        assert_eq!(correct_displays, square_displays);
    }
}
