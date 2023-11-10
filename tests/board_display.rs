use superdoku::*;

use pretty_assertions::assert_eq;


fn starting_game_representation_looks_right() {
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
fn game_in_progress_looks_right() {
    let mut board = Board::default();
    board.try_collapse(Number::Five, (0, 2));
    assert_eq!(
        vec![
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
            "1 | ? ? 5 | ? ? ? | ? ? ? | \n",
            "  |-------|-------|-------| \n",
            "    a b c   d e f   g h i   \n"
        ]
        .concat(),
        format!("{board}")
    );

    board.try_collapse(Number::One, to_location("c6"));
    board.try_collapse(Number::Seven, to_location("c7"));
    board.try_collapse(Number::Six, to_location("c5"));
    board.try_collapse(Number::Four, to_location("c4"));
    board.try_collapse(Number::Nine, to_location("c9"));
    board.try_collapse(Number::Three, to_location("c2"));
    board.try_collapse(Number::Eight, to_location("c3"));
    board.try_collapse(Number::Seven, to_location("c8"));
    board.try_collapse(Number::Nine, to_location("a3"));
    board.try_collapse(Number::Six, to_location("a2"));
    board.try_collapse(Number::Seven, to_location("b2"));
    board.try_collapse(Number::One, to_location("b1"));
    board.try_collapse(Number::Four, to_location("a1"));
    board.try_collapse(Number::Two, to_location("b3"));
    board.try_collapse(Number::Nine, to_location("b6"));
    board.try_collapse(Number::Five, to_location("b5"));
    board.try_collapse(Number::Eight, to_location("b4"));
    board.try_collapse(Number::Six, to_location("b8"));
    board.try_collapse(Number::Four, to_location("b9"));
    board.try_collapse(Number::Three, to_location("b7"));
    board.try_collapse(Number::Three, to_location("a4"));
    board.try_collapse(Number::Two, to_location("a5"));
    board.try_collapse(Number::Seven, to_location("a6"));
    board.try_collapse(Number::One, to_location("a9"));
    board.try_collapse(Number::Five, to_location("a8"));
    board.try_collapse(Number::Eight, to_location("a7"));
    board.try_collapse(Number::Five, to_location("g7"));
    board.try_collapse(Number::Nine, to_location("h7"));
    board.try_collapse(Number::Seven, to_location("e7"));
    board.try_collapse(Number::One, to_location("i7"));
    board.try_collapse(Number::Six, to_location("d7"));
    board.try_collapse(Number::Four, to_location("f7"));
    board.try_collapse(Number::Five, to_location("f9"));
    board.try_collapse(Number::Three, to_location("e9"));
    board.try_collapse(Number::Two, to_location("d9"));
    board.try_collapse(Number::Seven, to_location("h9"));
    board.try_collapse(Number::Eight, to_location("g9"));
    board.try_collapse(Number::Six, to_location("i9"));
    board.try_collapse(Number::Three, to_location("g8"));
    board.try_collapse(Number::Two, to_location("h8"));
    board.try_collapse(Number::Four, to_location("i8"));
    board.try_collapse(Number::Eight, to_location("h1"));
    board.try_collapse(Number::Three, to_location("h5"));
    board.try_collapse(Number::Nine, to_location("d8"));
    board.try_collapse(Number::Eight, to_location("f8"));
    board.try_collapse(Number::One, to_location("e8"));
    board.try_collapse(Number::Three, to_location("d1"));
    board.try_collapse(Number::Seven, to_location("f3"));
    board.try_collapse(Number::Five, to_location("i3"));
    board.try_collapse(Number::Four, to_location("h2"));
    board.try_collapse(Number::Five, to_location("h6"));
    board.try_collapse(Number::Six, to_location("h4"));
    board.try_collapse(Number::One, to_location("h3"));
    board.try_collapse(Number::Four, to_location("d3"));
    board.try_collapse(Number::Six, to_location("g3"));

    assert_eq!(
        vec![
            "                            \n",
            "  |-------|-------|-------| \n",
            "9 | 1 4 9 | 2 3 5 | 8 7 6 | \n",
            "8 | 5 6 0 | 9 1 8 | 3 2 4 | \n",
            "7 | 8 3 7 | 6 0 4 | 5 9 1 | \n",
            "  |-------|-------|-------| \n",
            "6 | 7 9 1 | ! ? ? | ? 5 ? | \n",
            "5 | 2 5 6 | ? ? ? | ? 3 ? | \n",
            "4 | 3 8 4 | ? ? ? | ? 6 ? | \n",
            "  |-------|-------|-------| \n",
            "3 | 9 2 8 | 4 0 7 | 6 1 5 | \n",
            "2 | 6 7 3 | ? ? ? | ? 4 ? | \n",
            "1 | 4 1 5 | 3 ? ? | ? 8 ? | \n",
            "  |-------|-------|-------| \n",
            "    a b c   d e f   g h i   \n"
        ]
        .concat(),
        format!("{board}")
    );
}

fn to_location(location: &str) -> (usize, usize) {
    (
        to_coordinate(location.as_bytes()[1]),
        to_coordinate(location.as_bytes()[0]),
    )
}

fn to_coordinate(coordinate: u8) -> usize {
    match coordinate {
        b'1'..=b'9' => (coordinate - b'1') as usize,
        b'a'..=b'i' => (coordinate - b'a') as usize,
        _ => panic!("Failed to create coordinate."),
    }
}
