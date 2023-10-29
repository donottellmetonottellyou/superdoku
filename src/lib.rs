mod square;

use square::{Number, Square};

use anyhow::{anyhow, Context, Result};
use rand::prelude::*;

use std::{cmp::Ordering, fmt::Display};

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Square; 9]; 9],
}
impl Board {
    pub fn is_solved(&self) -> bool {
        self.board
            .iter()
            .flat_map(|row| row.iter())
            .all(|square| square.superposition_number().is_err())
    }

    pub fn random_collapse(&mut self) -> Result<(Number, (usize, usize))> {
        let mut rng = thread_rng();

        let location = *self
            .find_lowest_superpositions()
            .context("Failed to find lowest superpositions")?
            .choose(&mut rng)
            .context("find_lowest_superpositions() inexplicably returned an empty Vec")?;

        let number = self.board[location.0][location.1]
            .collapse_random()
            .with_context(|| format!("Failed to randomly collapse square at {location:?}"))?;

        self.propagate_collapse(number, location)
            .with_context(|| format!("Failed to propagate collapse of {location:?} to {number}"))
            .context("Board is probably in an invalid state")?;

        Result::Ok((number, location))
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn try_collapse(&mut self, number: Number, location: (usize, usize)) -> Result<bool> {
        if let Ok(()) = self.board[location.0][location.1].collapse(number) {
            self.propagate_collapse(number, location)
                .with_context(|| {
                    format!("Failed to propagate collapse of {location:?} to {number}")
                })
                .context("Board is probably in an invalid state")?;

            Result::Ok(true)
        } else {
            Result::Ok(false)
        }
    }

    fn find_lowest_superpositions(&self) -> Result<Vec<(usize, usize)>> {
        let mut lowest_superpositions = Vec::new();
        let mut lowest_number = 9;

        self.board.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, square)| {
                if let Ok(superposition_number) = square.superposition_number() {
                    match superposition_number.cmp(&lowest_number) {
                        Ordering::Equal => lowest_superpositions.push((i, j)),
                        Ordering::Greater => {}
                        Ordering::Less => {
                            lowest_number = superposition_number;
                            lowest_superpositions.clear();
                            lowest_superpositions.push((i, j));
                        }
                    }
                }
            })
        });

        if lowest_number == 0 {
            Result::Err(anyhow!(
                "Board found to be in an invalid state: Lowest superposition is zero"
            ))
        } else {
            Result::Ok(lowest_superpositions)
        }
    }

    fn find_neighbor_locations(location: (usize, usize)) -> [(usize, usize); 20] {
        let mut neighbors = [(0, 0); 20];
        let mut neighbors_iter = neighbors.iter_mut();

        let location_box = (location.0 / 3, location.1 / 3);
        let location_box_corner = (location_box.0 * 3, location_box.1 * 3);

        // We find the neighbors in the same box.
        for i in 0..3 {
            for j in 0..3 {
                let box_location = (location_box_corner.0 + i, location_box_corner.1 + j);
                if box_location == location {
                    continue;
                }

                *neighbors_iter
                    .next()
                    .context("Ran out of neighbor spaces while searching box")
                    .unwrap() = box_location
            }
        }

        // We find the neighbors in the same row.
        for j in 0..9 {
            if location_box.1 == j / 3 {
                continue;
            }

            *neighbors_iter
                .next()
                .context("Ran out of neighbor spaces while searching row")
                .unwrap() = (location.0, j);
        }

        // We find the neighbors in the same column.
        for i in 0..9 {
            if location_box.0 == i / 3 {
                continue;
            }

            *neighbors_iter
                .next()
                .context("Ran out of neighbor spaces while searching column")
                .unwrap() = (i, location.1)
        }

        neighbors
    }

    fn propagate_collapse(&mut self, number: Number, location: (usize, usize)) -> Result<()> {
        for location in Self::find_neighbor_locations(location) {
            self.board[location.0][location.1]
                .remove(number)
                .with_context(|| format!("Failed to remove {number} at location {location:?}"))?;
        }

        Result::Ok(())
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
        let mut row_reversed_board_iter = self.board.iter().rev().flat_map(|row| row.iter());

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
                                .context("Fatally failed to display board")
                                .unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    use std::collections::HashSet;

    #[test]
    fn square_terminal_representation_looks_right() {
        let correct_display = vec![
            "                            \n",
            "  |-------|-------|-------| \n",
            "9 | ? ? ? | ? ? ? | ? ? ? | \n",
            "8 | ? ? ? | ? ? ? | ? ? ? | \n",
            "7 | ? ? ? | ? ? ? | ? ? ? | \n",
            "  |-------|-------|-------| \n",
            "6 | ? ? ? | ? ? ? | ? ? ? | \n",
            "5 | ? ? ? | ? ? ? | ? ? ? | \n",
            "4 | ? ? ? | ? ? ? | ? ? ? | \n",
            "  |-------|-------|-------| \n",
            "3 | ? ? ? | ? ? ? | ? ? ? | \n",
            "2 | ? ? ? | ? ? ? | ? ? ? | \n",
            "1 | ? ? ? | ? ? ? | ? ? ? | \n",
            "  |-------|-------|-------| \n",
            "    a b c   d e f   g h i   \n",
        ]
        .concat();

        assert_eq!(correct_display, format!("{}", Board::default()));
    }

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
