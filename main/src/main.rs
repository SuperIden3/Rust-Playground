// imports
use future::*;
use ops::*;
use std::io::*;
use std::*;
use std::time::*;
use fmt::*;
use result::*;
use error::*;
use tokio;

// constants
const PI: f64 = 3.141592653589793;

// structures
struct IRead {
    length: usize,
    input: String,
}

// #[tokio::main]
pub fn main() {
    let start: Instant = Instant::now();

    // main
    let str: Box<String> = Box::new(String::from("Hello, World!"));
    println!("{:#?}", str);

    let end: Duration = start.elapsed();
    println!("Code: {:#?}ms", end.as_millis() as f64);
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
