// imports
use future::{self};
use ops::{self, *};
use std::io::{self, *};
use std::{self, *};

// constants
const PI: f64 = 3.141592653589793;

// structures
struct IRead {
    length: usize,
    input: String,
}

#[tokio::main]
async fn main() {
    // main
    let number: f64 = get_line("What is the approximation of PI? ")
        .unwrap()
        .input
        .parse::<f64>()
        .unwrap();
    let condition: bool = number == (PI.mul(100.0)).round() / 100.0;
    match condition {
        true => {
            println!("Correct!");
        }
        false => {
            println!("Wrong!");
        }
    }
}

// functions
fn get_line(question: &str) -> Result<IRead> {
    let stdin: io::Stdin = io::stdin();
    let mut stdout: io::Stdout = io::stdout();

    let mut _input: String = String::new();
    print!("{}", question);

    stdout
        .flush()
        .expect("An error occurred while flushing stdout");
    let size: usize = stdin
        .read_line(&mut _input)
        .expect("An error occured while getting input");

    Ok(IRead {
        length: size,
        input: _input.trim().to_string(),
    })
}
