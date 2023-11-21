pub enum StartChoice {
    StartGame,
    Quit,
}
impl StartChoice {
    pub fn get() -> StartChoice {
        let mut choice = readln!("Do you want to start a new game? (Y/n): ");

        choice.make_ascii_uppercase();

        match choice.as_str() {
            "" | "Y" | "YES" => StartChoice::StartGame,
            "N" | "NO" => StartChoice::Quit,
            _ => {
                println!("I did not understand that.");
                Self::get()
            }
        }
    }
}
