mod square;

use square::{Number, Square};

use anyhow::{Context, Result};

use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Square; 9]; 9],
}
impl Board {
    pub fn try_collapse(&mut self, number: Number, location: (usize, usize)) -> Result<()> {
        todo!()
    }

    pub fn random_collapse(&mut self) -> Result<(Number, (usize, usize))> {
        todo!()
    }

    fn propogate_collapse(&mut self, number: Number, location: (usize, usize)) {
        todo!()
    }

    fn find_neighbor_locations(location: (usize, usize)) -> [(usize, usize); 19] {
        let mut neighbors = [(0, 0); 19];
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
