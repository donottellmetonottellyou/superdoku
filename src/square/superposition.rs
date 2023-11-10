use super::Number;

use rand::prelude::*;

use std::{collections::BTreeSet, fmt::Display};

#[derive(Clone, Debug)]
pub struct Superposition {
    superposition: BTreeSet<Number>,
}
impl Superposition {
    pub fn contains(&self, number: Number) -> bool {
        self.superposition.contains(&number)
    }

    pub fn collapse_random(&self) -> Option<Number> {
        self.superposition
            .iter()
            .copied()
            .collect::<Vec<_>>()
            .choose(&mut thread_rng())
            .cloned()
    }

    pub fn remove(&mut self, number: Number) -> bool {
        self.superposition.remove(&number)
    }

    pub fn superposition_number(&self) -> usize {
        self.superposition.len()
    }
}
impl Default for Superposition {
    fn default() -> Self {
        Self {
            superposition: BTreeSet::from(Number::ALL),
        }
    }
}
impl Display for Superposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.superposition.len() {
            0 => "0",
            1 => "!",
            _ => "?",
        })
    }
}
