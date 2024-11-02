// imports
use borrow::{self, *};
use async_stream::{self, *};
use future::*;
use ops::{self, *};
use rand::{self, *};
use std::io::{self, *};
use std::*;
use std::{self, *};
use tokio::runtime::{self, *};
use tokio::*;
use tokio::{self, *};

// constants
const PI: f64 = 3.141592653589793;

// structures
/**
Struct that holds a `String` and the length of it.

Usually used with working with input.

```rs
let a = IRead {
  input: "Hello World!",
  length: 12
};
println!("{:?}", a);
```
 */
struct IRead {
    length: usize,
    input: String,
}

#[tokio::main]
pub async fn main() {
    // main
    println!("{:#?}", ask("Input: ").unwrap().input);
}

// functions
/**
Gets input from `Stdin` with an optional question.

Returns a `Result<IRead, io::Error>` where `IRead` is `struct { length: usize, input: String }`.
 */
fn ask(question: &str) -> io::Result<IRead> {
    let stdin: Stdin = io::stdin();
    let mut stdout: Stdout = io::stdout();

    let mut _input: String = String::new();
    print!("{}", question);

    let result1: result::Result<(), io::Error> = stdout.flush();
    let result2: result::Result<usize, io::Error> = stdin.read_line(&mut _input);
    match result1 {
        Err(e) => {
            return Err(e);
        }
        Ok(_) => {}
    }
    let size: usize;
    match result2 {
        Err(e) => {
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

/**
Picks a random element from an array.

Returns a reference to the chosen element.
 */
fn pick_random<T>(arr: &[T]) -> &T {
    let mut rng: rngs::ThreadRng = rand::thread_rng();
    let index: usize = rng.gen_range(0..arr.len());
    return &arr[index];
}
