#[warn(clippy::all, clippy::pedantic)]
//use std::env;
mod lexer;
use lexer::tokenize;
fn main() {
    //let args: Vec<String> = env::args().collect();

    let code = String::from(
        "let x: Int = 42;
let y = x + 10;
if (y == 52) {
    return y;
}",
    );
    println!("{}", code);
    println!("--------------------------------------------------");
    for token in tokenize(&code) {
        println!("{:?}", token);
    }
}
