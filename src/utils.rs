use std::io::Write;

#[allow(dead_code)]
pub fn get_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    loop {
        print!("{prompt}");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            continue;
        }

        let input = input.trim();

        match input.parse::<T>() {
            Ok(value) => return value,
            Err(e) => {
                println!("unavaiable input: {:?}", e);
            }
        }
    }
}

