#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    let mut cmd = String::new();
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut cmd) {
        Ok(..) => {
            let cmd = cmd.trim();
            println!("{cmd}: command not found");
        }
        Err(_) => println!("An internal error occured!"),
    }
}
