mod number;
mod superposition;

pub use number::Number;
use superposition::Superposition;

use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Square {
    Incomplete(Superposition),
    PlayerMove(Number),
}
impl Square {
    pub fn collapse_random(&mut self) -> Option<Number> {
        match self {
            Self::PlayerMove(_collapsed) => None,
            Self::Incomplete(superposition) => {
                let number = superposition.collapse_random()?;

                *self = Self::PlayerMove(number);

                Some(number)
            }
        }
    }

    pub fn collapsed_number(&self) -> Option<Number> {
        match self {
            Self::PlayerMove(collapsed) => Some(*collapsed),
            Self::Incomplete(_superposition) => None,
        }
    }

    pub fn remove(&mut self, number: Number) -> bool {
        match self {
            Self::PlayerMove(_collapsed) => false,
            Self::Incomplete(superposition) => superposition.remove(number),
        }
    }

    pub fn superposition_number(&self) -> Option<usize> {
        match self {
            Self::PlayerMove(_collapsed) => None,
            Self::Incomplete(superposition) => Some(superposition.superposition_number()),
        }
    }

    pub fn try_collapse(&mut self, number: Number) -> bool {
        match self {
            Self::PlayerMove(_collapsed) => false,
            Self::Incomplete(superposition) => {
                if superposition.contains(number) {
                    *self = Self::PlayerMove(number);
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn undo_collapse(&mut self) -> bool {
        match self {
            Self::PlayerMove(_collapsed) => {
                *self = Self::Incomplete(Superposition::default());
                true
            }
            Self::Incomplete(_superposition) => false,
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
            Self::PlayerMove(displayable) => displayable.fmt(f),
            Self::Incomplete(displayable) => displayable.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn square_displays_correctly() {
        let square_displays: Vec<_> = vec![
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
        .map(|square| format!("{square}"))
        .collect();

        let correct_displays = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "?", "!", "0"];

        assert_eq!(correct_displays, square_displays);
    }
}
