use crate::cli::io::DID_NOT_UNDERSTAND;

use superdoku::Number;

pub enum GameChoice {
    Move(Number, (usize, usize)),
    MoveRandom,
    Undo((usize, usize)),
    End,
}
impl GameChoice {
    pub fn get() -> GameChoice {
        println!("Move: m number position (m 1 a1)");
        println!("Move Randomly: r");
        println!("Undo: u position (u a1)");
        println!("Quit: q");
        let mut choice = readln!("Make a choice: ");

        choice.make_ascii_uppercase();

        match choice.as_str() {
            "R" => return GameChoice::MoveRandom,
            "Q" => return GameChoice::End,
            _ => {},
        }

        match choice.as_bytes().first() {
            Some(b'M') => {
                if let Some(choice) = Self::parse_move(choice[1..].trim()) {
                    return choice;
                }
            },
            Some(b'U') => {
                if let Some(choice) = Self::parse_undo(choice[1..].trim()) {
                    return choice;
                }
            },
            Some(_) | None => {},
        }

        println!("{DID_NOT_UNDERSTAND}");

        Self::get()
    }

    fn parse_move(choice: &str) -> Option<GameChoice> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 2 {
            return None;
        }

        Some(GameChoice::Move(
            super::parse_number(choices[0])?,
            super::parse_location(choices[1])?,
        ))
    }

    fn parse_undo(choice: &str) -> Option<GameChoice> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 1 {
            return None;
        }

        Some(GameChoice::Undo(super::parse_location(choices[0])?))
    }
}
