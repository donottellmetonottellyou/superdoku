mod square;

pub use square::Number;
use square::Square;

use rand::prelude::*;

use std::{cmp::Ordering, collections::VecDeque, fmt::Display};

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Square; 9]; 9],
}
impl Board {
    pub fn is_solved(&self) -> bool {
        self.board
            .iter()
            .flat_map(|row| row.iter())
            .all(|square| square.collapsed_number().is_some())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn try_move(
        &mut self,
        number: Number,
        location: (usize, usize),
    ) -> bool {
        if self.get_mut(location).try_move(number) {
            self.propagate_collapse(number, location);

            true
        } else {
            false
        }
    }

    pub fn try_set(
        &mut self,
        number: Number,
        location: (usize, usize),
    ) -> bool {
        if self.get_mut(location).try_set(number) {
            self.propagate_collapse(number, location);

            true
        } else {
            false
        }
    }

    pub fn try_random_move(&mut self) -> Option<(Number, (usize, usize))> {
        let mut rng = thread_rng();

        let location = *self
            .find_lowest_superpositions()?
            .choose(&mut rng)
            .expect(
            "find_lowest_superpositions() inexplicably returned an empty Vec",
        );

        let number = self.get_mut(location).try_random_move()?;

        self.propagate_collapse(number, location);

        Some((number, location))
    }

    pub fn try_random_set(&mut self) -> Option<(Number, (usize, usize))> {
        let mut rng = thread_rng();

        let location = *self
            .find_highest_superpositions()?
            .choose(&mut rng)
            .expect(
            "find_highest_superpositions() inexplicably returned an empty Vec",
        );

        let number = self.get_mut(location).try_random_set()?;

        self.propagate_collapse(number, location);

        Some((number, location))
    }

    pub fn try_undo_move(&mut self, location: (usize, usize)) -> bool {
        if !self.get_mut(location).try_undo_move() {
            return false;
        };

        self.propagate_undo_collapse(location);

        true
    }

    pub fn try_undo_set(&mut self, location: (usize, usize)) -> bool {
        if !self.get_mut(location).try_undo_set() {
            return false;
        }

        self.propagate_undo_collapse(location);

        true
    }

    fn find_highest_superpositions(&self) -> Option<Vec<(usize, usize)>> {
        let mut highest_superpositions = Vec::new();
        let mut highest_number = 0;

        for (i, j, square) in self {
            if let Some(superposition_number) = square.superposition_number() {
                match superposition_number.cmp(&highest_number) {
                    Ordering::Equal => highest_superpositions.push((i, j)),
                    Ordering::Greater => {
                        highest_number = superposition_number;
                        highest_superpositions.clear();
                        highest_superpositions.push((i, j));
                    },
                    Ordering::Less => {},
                }
            }
        }

        if highest_number == 0 {
            None
        } else {
            Some(highest_superpositions)
        }
    }

    fn find_lowest_superpositions(&self) -> Option<Vec<(usize, usize)>> {
        let mut lowest_superpositions = Vec::new();
        let mut lowest_number = 9;

        for (i, j, square) in self {
            if let Some(superposition_number) = square.superposition_number() {
                match superposition_number.cmp(&lowest_number) {
                    Ordering::Equal => lowest_superpositions.push((i, j)),
                    Ordering::Greater => {},
                    Ordering::Less => {
                        lowest_number = superposition_number;
                        lowest_superpositions.clear();
                        lowest_superpositions.push((i, j));
                    },
                }
            }
        }

        if lowest_number == 0 {
            None
        } else {
            Some(lowest_superpositions)
        }
    }

    fn get(&self, location: (usize, usize)) -> &Square {
        self.board
            .get(location.0)
            .unwrap_or_else(|| {
                panic!("Failed to get ref to square at {location:?}: not valid location.")
            })
            .get(location.1)
            .unwrap_or_else(|| {
                panic!("Failed to get ref to square at {location:?}: not valid location.")
            })
    }

    fn get_mut(&mut self, location: (usize, usize)) -> &mut Square {
        self.board
            .get_mut(location.0)
            .unwrap_or_else(|| {
                panic!("Failed to get mut ref to square at {location:?}: not valid location.")
            })
            .get_mut(location.1)
            .unwrap_or_else(|| {
                panic!("Failed to get mut ref to square at {location:?}: not valid location.")
            })
    }

    fn propagate_collapse(&mut self, number: Number, location: (usize, usize)) {
        for location in Self::find_neighbor_locations(location) {
            self.get_mut(location).remove(number);
        }
    }

    fn propagate_undo_collapse(&mut self, location: (usize, usize)) {
        for neighbor in Self::find_neighbor_locations(location) {
            if let Some(collapsed) =
                self.board[neighbor.0][neighbor.1].collapsed_number()
            {
                self.get_mut(location).remove(collapsed);
            } else {
                self.update_superposition(neighbor);
            }
        }
    }

    fn update_superposition(&mut self, location: (usize, usize)) {
        *self.get_mut(location) = Square::default();

        for neighbor in Self::find_neighbor_locations(location) {
            if let Some(collapsed_number) =
                self.get(neighbor).collapsed_number()
            {
                self.get_mut(location).remove(collapsed_number);
            }
        }
    }

    fn find_neighbor_locations(
        location: (usize, usize),
    ) -> [(usize, usize); 20] {
        let mut neighbors = [(0, 0); 20];
        let mut neighbors_iter = neighbors.iter_mut();

        let location_box = (location.0 / 3, location.1 / 3);
        let location_box_corner = (location_box.0 * 3, location_box.1 * 3);

        // We find the neighbors in the same box.
        for i in 0..3 {
            for j in 0..3 {
                let box_location =
                    (location_box_corner.0 + i, location_box_corner.1 + j);
                if box_location == location {
                    continue;
                }

                *neighbors_iter
                    .next()
                    .expect("Ran out of neighbor spaces while searching box") =
                    box_location
            }
        }

        // We find the neighbors in the same row.
        for j in 0..9 {
            if location_box.1 == j / 3 {
                continue;
            }

            *neighbors_iter
                .next()
                .expect("Ran out of neighbor spaces while searching row") =
                (location.0, j);
        }

        // We find the neighbors in the same column.
        for i in 0..9 {
            if location_box.0 == i / 3 {
                continue;
            }

            *neighbors_iter
                .next()
                .expect("Ran out of neighbor spaces while searching column") =
                (i, location.1)
        }

        neighbors
    }
}
impl Default for Board {
    fn default() -> Self {
        Self {
            board: vec![vec![Square::default(); 9].try_into().unwrap(); 9]
                .try_into()
                .unwrap(),
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_reversed_board_iter =
            self.board.iter().rev().flat_map(|row| row.iter());

        f.write_str("                            \n")?;
        f.write_str("  |-------|-------|-------| \n")?;
        //           9 | ? ? ? | ? ? ? | ? ? ? |

        for box_row in 0..3 {
            for row in 0..3 {
                f.write_str(&format!("{} | ", 9 - ((box_row * 3) + row)))?;
                for _square_triplet in 0..3 {
                    for _square in 0..3 {
                        f.write_str(&format!(
                            "{} ",
                            row_reversed_board_iter
                                .next()
                                .expect("Fatally failed to display board")
                        ))?;
                    }
                    f.write_str("| ")?;
                }
                f.write_str("\n")?;
            }
            //           1 | ? ? ? | ? ? ? | ? ? ? |
            f.write_str("  |-------|-------|-------| \n")?;
        }

        //             |-------|-------|-------|
        f.write_str("    a b c   d e f   g h i   \n")
    }
}
impl<'a> IntoIterator for &'a Board {
    type IntoIter = BoardSquareIter<'a>;
    type Item = (usize, usize, &'a Square);

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { board: self, x: 0, y: 0 }
    }
}
pub struct BoardSquareIter<'a> {
    board: &'a Board,
    x: usize,
    y: usize,
}
impl<'a> Iterator for BoardSquareIter<'a> {
    type Item = (usize, usize, &'a Square);
    fn next(&mut self) -> Option<Self::Item> {
        let next;

        if let Some(row) = self.board.board.get(self.x) {
            if let Some(square) = row.get(self.y) {
                next = Some((self.x, self.y, square));
            } else if let Some(Some(square)) = {
                self.y = 0;
                self.x += 1;

                self.board.board.get(self.x).map(|row| row.get(self.y))
            } {
                next = Some((self.x, self.y, square));
            } else {
                next = None
            }
        } else {
            next = None
        }

        self.y += 1;

        next
    }
}

struct Step {
    number: Number,
    location: (usize, usize),
}

struct GuessStep {
    wrong_starts: Vec<Step>,
    steps: VecDeque<Step>,
}

struct Solution {
    steps: VecDeque<GuessStep>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use std::collections::HashSet;

    #[test]
    fn find_neighbor_locations_finds_correct_locations() {
        // 5e or e5
        let location = (4, 4);
        let correct_neighbors = HashSet::from([
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 3),
            (4, 5),
            (5, 3),
            (5, 4),
            (5, 5),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 6),
            (4, 7),
            (4, 8),
            (0, 4),
            (1, 4),
            (2, 4),
            (6, 4),
            (7, 4),
            (8, 4),
        ]);

        assert_eq!(
            correct_neighbors,
            HashSet::from(Board::find_neighbor_locations(location))
        );
    }
}
