mod number;
mod superposition;

pub use number::Number;
use superposition::Superposition;

use anyhow::{anyhow, Context, Result};

use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Square {
    Number(Number),
    Superposition(Superposition),
}
impl Square {
    pub fn collapse_random(&mut self) -> Option<Number> {
        match self {
            Self::Number(_collapsed) => None,
            Self::Superposition(superposition) => {
                let number = superposition.collapse_random()?;

                *self = Self::Number(number);

                Some(number)
            }
        }
    }

    pub fn collapsed_number(&self) -> Option<Number> {
        match self {
            Self::Number(collapsed) => Some(*collapsed),
            Self::Superposition(_superposition) => None,
        }
    }

    pub fn remove(&mut self, number: Number) -> bool {
        match self {
            Self::Number(_collapsed) => false,
            Self::Superposition(superposition) => superposition.remove(number),
        }
    }

    pub fn superposition_number(&self) -> Option<usize> {
        match self {
            Self::Number(collapsed) => None,
            Self::Superposition(superposition) => Some(superposition.superposition_number()),
        }
    }

    pub fn try_collapse(&mut self, number: Number) -> bool {
        match self {
            Self::Number(collapsed) => false,
            Self::Superposition(superposition) => {
                if superposition.contains(number) {
                    *self = Self::Number(number);
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn undo_collapse(&mut self) -> bool {
        match self {
            Self::Number(_collapsed) => {
                *self = Self::Superposition(Superposition::default());
                true
            }
            Self::Superposition(_superposition) => false,
        }
    }
}
impl Default for Square {
    fn default() -> Self {
        Self::Superposition(Superposition::default())
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(displayable) => displayable.fmt(f),
            Self::Superposition(displayable) => displayable.fmt(f),
        }
    }
}
