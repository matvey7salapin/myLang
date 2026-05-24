mod lexer;
#[warn(clippy::all, clippy::pedantic)]
mod parser;
mod types;

use lexer::tokenize;
fn main() {
    //let args: Vec<String> = env::args().collect();

    let code = String::from(
        "let x: Int = 42;
let c: string = 'a'; 
let y = x + 10;
if (y == 5_2.0235) {
    return y;
}",
    );
    println!("{}", code);
    println!("--------------------------------------------------");
    for token in tokenize(&code) {
        println!("{:?}", token);
    }
}
