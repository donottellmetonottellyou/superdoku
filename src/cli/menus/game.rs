use superdoku::Number;

pub enum GameChoice {
    Move(Number, (usize, usize)),
    Undo((usize, usize)),
    MoveRandom,
    End,
}
impl GameChoice {
    pub fn get() -> GameChoice {
        println!("Move: m number position (m 1 a1)");
        println!("Undo: u position (u a1)");
        println!("Move Randomly: r");
        println!("Quit: q");
        let mut choice = readln!("Make a choice: ");

        choice.make_ascii_uppercase();

        match choice.as_str() {
            "R" => return GameChoice::MoveRandom,
            "Q" => return GameChoice::End,
            _ => {}
        }

        match choice.as_bytes().first() {
            Some(b'M') => {
                if let Some(choice) = Self::parse_move(choice[1..].trim()) {
                    return choice;
                }
            }
            Some(b'U') => {
                if let Some(choice) = Self::parse_undo(choice[1..].trim()) {
                    return choice;
                }
            }
            Some(_) | None => {}
        }

        println!("I couldn't understand that!");

        Self::get()
    }

    fn parse_move(choice: &str) -> Option<GameChoice> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 2 {
            return None;
        }

        Some(GameChoice::Move(
            Self::parse_number(choices[0])?,
            Self::parse_location(choices[1])?,
        ))
    }

    fn parse_undo(choice: &str) -> Option<GameChoice> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 1 {
            return None;
        }

        Some(GameChoice::Undo(Self::parse_location(choices[0])?))
    }

    fn parse_location(location: &str) -> Option<(usize, usize)> {
        let coordinates: Vec<_> = location.bytes().collect();
        if coordinates.len() != 2 {
            return None;
        }

        Some(
            match (
                coordinates[0].is_ascii_digit(),
                coordinates[1].is_ascii_digit(),
            ) {
                (true, false) => (
                    Self::parse_coordinate(coordinates[0])?,
                    Self::parse_coordinate(coordinates[1])?,
                ),
                (false, true) => (
                    Self::parse_coordinate(coordinates[1])?,
                    Self::parse_coordinate(coordinates[0])?,
                ),
                (true, true) | (false, false) => return None,
            },
        )
    }

    fn parse_coordinate(coordinate: u8) -> Option<usize> {
        Some(match coordinate {
            b'1'..=b'9' => (coordinate - 49) as usize,
            b'A'..=b'I' => (coordinate - 65) as usize,
            _ => return None,
        })
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
}
