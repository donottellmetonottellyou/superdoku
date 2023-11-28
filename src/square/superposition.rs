use super::Number;

use rand::prelude::*;

use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Superposition {
    superposition: Vec<Number>,
}
impl Superposition {
    pub fn contains(&self, number: Number) -> bool {
        self.superposition.binary_search(&number).is_ok()
    }

    pub fn collapse_random(&self) -> Option<Number> {
        self.superposition.choose(&mut thread_rng()).cloned()
    }

    pub fn remove(&mut self, number: Number) -> bool {
        if let Ok(index) = self.superposition.binary_search(&number) {
            self.superposition.remove(index);
            true
        } else {
            false
        }
    }

    pub fn superposition_number(&self) -> usize {
        self.superposition.len()
    }
}
impl Default for Superposition {
    fn default() -> Self {
        Self { superposition: Number::ALL.into() }
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
