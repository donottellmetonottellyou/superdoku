mod game;
mod setup;
mod start;

pub use game::*;
pub use setup::*;
pub use start::*;

use superdoku::Number;

fn parse_coordinate(coordinate: u8) -> Option<usize> {
    Some(match coordinate {
        b'1'..=b'9' => (coordinate - 49) as usize,
        b'A'..=b'I' => (coordinate - 65) as usize,
        _ => return None,
    })
}

fn parse_location(location: &str) -> Option<(usize, usize)> {
    let coordinates: Vec<_> = location.bytes().collect();
    if coordinates.len() != 2 {
        return None;
    }

    Some(
        match (coordinates[0].is_ascii_digit(), coordinates[1].is_ascii_digit())
        {
            (true, false) => (
                parse_coordinate(coordinates[0])?,
                parse_coordinate(coordinates[1])?,
            ),
            (false, true) => (
                parse_coordinate(coordinates[1])?,
                parse_coordinate(coordinates[0])?,
            ),
            (true, true) | (false, false) => return None,
        },
    )
}

fn parse_number(number: &str) -> Option<Number> {
    let number: Vec<_> = number.bytes().collect();
    if number.len() != 1 {
        return None;
    }
    let number = number.into_iter().next()?;

    Some(match number {
        b'1' => Number::One,
        b'2' => Number::Two,
        b'3' => Number::Three,
        b'4' => Number::Four,
        b'5' => Number::Five,
        b'6' => Number::Six,
        b'7' => Number::Seven,
        b'8' => Number::Eight,
        b'9' => Number::Nine,
        _ => return None,
    })
}
