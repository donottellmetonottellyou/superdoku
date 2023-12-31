pub enum Choice {
    StartGame,
    Quit,
}

pub fn menu() -> Choice {
    let mut choice = readln!("Do you want to start a new game? (Y/n): ");

    choice.make_ascii_uppercase();

    match choice.as_str() {
        "" | "Y" | "YES" => Choice::StartGame,
        "N" | "NO" => Choice::Quit,
        _ => {
            println!("I did not understand that.");
            menu()
        }
    }
}
