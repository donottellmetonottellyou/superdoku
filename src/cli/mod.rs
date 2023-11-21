#[macro_use]
mod io;
mod menus;

use superdoku::Board;

pub fn main() {
    io::clear();
    'main: loop {
        println!("Welcome to Superdoku!");
        let choice = menus::StartChoice::get();

        io::clear();
        match choice {
            menus::StartChoice::StartGame => {}
            menus::StartChoice::Quit => break 'main,
        }
        let mut board = Board::default();

        while !board.is_solved() {
            println!("{board}");
            let choice = menus::GameChoice::get();

            io::clear();
            match choice {
                menus::GameChoice::Move(number, location) => {
                    if board.try_move(number, location) {
                        println!("Successfully executed move.");
                    } else {
                        println!("Failed to execute move.");
                        println!(
                            "{number} was not a possibility at {}",
                            io::location_to_string(location)
                        );
                    }
                }
                menus::GameChoice::Undo(location) => {
                    if board.try_undo_move(location) {
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
                menus::GameChoice::MoveRandom => match board.try_random_move() {
                    Some((number, location)) => println!(
                        "Successfully chose {number} at {}",
                        io::location_to_string(location)
                    ),
                    None => {
                        println!("A random move is not possible at this time.");
                        println!("Try undoing a move.")
                    }
                },
                menus::GameChoice::End => continue 'main,
            }
        }

        // If 'game exits without returning early or continuing the outer 'main loop, we have solved
        // the board!
        println!("{board}");
        println!("The board was solved!");
    }
}
