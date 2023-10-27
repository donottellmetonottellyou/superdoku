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
    pub fn collapse(&mut self, number: Number) -> Result<()> {
        match self {
            Self::Number(collapsed) => Result::Err(anyhow!(
                "Square already collapsed into {collapsed}, cannot collapse into {number}"
            )),
            Self::Superposition(superposition) => {
                if superposition.contains(number) {
                    *self = Self::Number(number);
                    Result::Ok(())
                } else {
                    Result::Err(anyhow!("Square cannot collapse into {number}"))
                }
            }
        }
    }

    pub fn collapse_random(&mut self) -> Result<Number> {
        match self {
            Self::Number(collapsed) => Result::Err(anyhow!(
                "Square already collapsed into {collapsed}, cannot collapse into any new number"
            )),
            Self::Superposition(superposition) => superposition
                .collapse_random()
                .context("Failed to collapse superposition"),
        }
    }

    pub fn remove(&mut self, number: Number) -> Result<bool> {
        match self {
            Self::Number(collapsed) if *collapsed == number => {
                Result::Err(anyhow!("Tried to remove {number} from {number}"))
            }
            Self::Number(_) => Result::Ok(false),
            Self::Superposition(superposition) => Result::Ok(superposition.remove(number)),
        }
    }

    pub fn superposition_number(&self) -> Result<usize> {
        match self {
            Self::Number(collapsed) => Result::Err(anyhow!(
                "Square already collapsed into {collapsed}, it doesn't have a superposition number"
            )),
            Self::Superposition(superposition) => Result::Ok(superposition.superposition_number()),
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
