use crate::cli::io::DID_NOT_UNDERSTAND;

use superdoku::Number;

pub enum SetupChoice {
    Set(Number, (usize, usize)),
    SetRandom,
    Undo((usize, usize)),
    Finish,
    End,
}
impl SetupChoice {
    pub fn get() -> Self {
        println!("Set: s number position (s 1 a1)");
        println!("Set Randomly: r");
        println!("Undo: u position (u a1)");
        println!("Finish: f");
        println!("Quit: q");
        let mut choice = readln!("Make a choice: ");

        choice.make_ascii_uppercase();

        match choice.as_str() {
            "R" => return Self::SetRandom,
            "F" => return Self::Finish,
            "Q" => return Self::End,
            _ => {},
        }

        match choice.as_bytes().first() {
            Some(b'S') => {
                if let Some(choice) = Self::parse_set(choice[1..].trim()) {
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

    fn parse_set(choice: &str) -> Option<Self> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 2 {
            return None;
        }

        Some(Self::Set(
            super::parse_number(choices[0])?,
            super::parse_location(choices[1])?,
        ))
    }

    fn parse_undo(choice: &str) -> Option<Self> {
        let choices: Vec<_> = choice.split(' ').collect();
        if choices.len() != 1 {
            return None;
        }

        Some(Self::Undo(super::parse_location(choices[0])?))
    }
}
