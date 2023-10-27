mod square;

use square::Square;

use anyhow::Context;

use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Square; 9]; 9],
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
                for square_triplet in 0..3 {
                    for square in 0..3 {
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
}
