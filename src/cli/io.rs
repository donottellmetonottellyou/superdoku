pub static IO_ERROR: &str = "A fatal I/O error occurred";

pub fn location_to_string(location: (usize, usize)) -> String {
    format!(
        "{}{}",
        (location.1 as u8 + b'a') as char,
        (location.0 as u8 + b'1') as char
    )
}

macro_rules! readln {
    () => {{
        use crate::cli::io::IO_ERROR;

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect(IO_ERROR);

        input.trim().to_string()
    }};
    ($($to_be_formatted:tt)*) => {{
        use crate::cli::io::IO_ERROR;

        use std::io::Write;

        print!($($to_be_formatted)*);
        std::io::stdout().flush().expect(IO_ERROR);
        readln!()
    }};
}
