// imports
use future::{self};
use ops::{self, *};
use std::io::{self, *};
use std::{self, *};
use rand::*;

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
    println!("{:#?}", pick_random::<u8>([1, 2, 3, 4, 5]));
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

fn pick_random<T>(arr: &[T]) -> &T { 
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..arr.len());
    return &arr[index];
}
