use superdoku::*;

use pretty_assertions::assert_eq;

#[test]
fn starting_square_terminal_representation_looks_right() {
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
