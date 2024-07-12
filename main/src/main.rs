const PI: f64 = 3.141592653589793;

#[tokio::main]
async fn main() {
    println!("Hello, {}!", get_line("What's your name? "));
}

fn get_line(question: &str) -> String {
    use std::io::{self, Write};

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut _input = String::new();
    print!("{}", question);

    stdout.flush().unwrap();
    stdin.read_line(&mut _input).expect("Error getting input!");
    let _input = _input.trim();

    return String::from(_input);
}