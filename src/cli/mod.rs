#[macro_use]
mod io;
mod menus;

use superdoku::Board;

pub fn main() {
    io::clear();
    'main: loop {
        println!("Welcome to Superdoku!");
        let choice = menus::main::menu();

        io::clear();
        match choice {
            menus::main::Choice::StartGame => {}
            menus::main::Choice::Quit => break 'main,
        }
        let mut board = Board::default();

        while !board.is_solved() {
            println!("{board}");
            let choice = menus::game::menu();

            io::clear();
            match choice {
                menus::game::Choice::Move(number, location) => {
                    if board.try_collapse(number, location) {
                        println!("Successfully executed move.");
                    } else {
                        println!("Failed to execute move.");
                        println!(
                            "{number} was not a possibility at {}",
                            io::location_to_string(location)
                        );
                    }
                }
                menus::game::Choice::Undo(location) => {
                    if board.undo(location) {
                        println!(
                            "Successfully removed move at {}",
                            io::location_to_string(location)
                        );
                    } else {
                        println!(
                            "Failed to undo move at {}",
                            io::location_to_string(location)
                        );
                    }
                }
                menus::game::Choice::MoveRandom => match board.random_collapse() {
                    Some((number, location)) => println!(
                        "Successfully chose {number} at {}",
                        io::location_to_string(location)
                    ),
                    None => {
                        println!("A random move is not possible at this time.");
                        println!("Try undoing a move.")
                    }
                },
                menus::game::Choice::End => continue 'main,
            }
        }

        // If 'game exits without returning early or continuing the outer 'main loop, we have solved
        // the board!
        println!("{board}");
        println!("The board was solved!");
    }
}
