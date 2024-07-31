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

// #[tokio::main]
pub fn main() {
    // main
    println!("{:#?}", pick_random::<u8>(&[1, 2, 3, 4, 5]));
}

// functions
fn ask(question: &str) -> io::Result<IRead> {
    let stdin: Stdin = io::stdin();
    let mut stdout: Stdout = io::stdout();

    let mut _input: String = String::new();
    print!("{}", question);

    let result1: result::Result<(), io::Error> = stdout.flush();
    let result2: result::Result<usize, io::Error> = stdin
        .read_line(&mut _input);
    match result1 {
        Err(e) => {
            eprintln!("{:#?}", e);
            return Err(e);
        }
        Ok(_) => {}
    }
    let size: usize;
    match result2 {
        Err(e) => {
            eprintln!("{:#?}", e);
            return Err(e);
        }
        Ok(s) => {
            size = s;
        }
    }

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
