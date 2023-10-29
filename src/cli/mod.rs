#[macro_use]
mod io;
mod menus;

use superdoku::Board;

pub fn main() {
    'main: loop {
        println!("Welcome to Superdoku!");
        match menus::main::menu() {
            menus::main::Choice::StartGame => {}
            menus::main::Choice::Quit => break 'main,
        }
        let mut board = Board::default();

        'game: loop {
            println!("{board}");
            match menus::game::menu() {
                menus::game::Choice::Move(number, location) => {
                    match board.try_collapse(number, location) {
                        Ok(true) => println!("Successfully executed move."),
                        Ok(false) => {
                            println!("Failed to execute move.");
                            println!(
                                "{number} was not a possibility at {}",
                                io::location_to_string(location)
                            );
                        }
                        Err(error) => {
                            println!("{error}");
                            return;
                        }
                    }
                }
                menus::game::Choice::MoveRandom => match board.random_collapse() {
                    Ok((number, location)) => println!(
                        "Successfully chose {number} at {}",
                        io::location_to_string(location)
                    ),
                    Err(error) => {
                        println!("{error}");
                        return;
                    }
                },
                menus::game::Choice::End => break 'game,
            }
        }
    }
}
