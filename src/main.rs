pub mod interpreter;

use std::io;
use std::io::Write;

use interpreter::lexer::Lexer;

use crate::interpreter::interpreter::Interpreter;

fn main() {

    let welcome_str = "Welcome to calculator!
You can use:
'+' - for summation
'-' - for difference
'/' - for division
'%' - for modular division
'^' - for power
'q' or 'Ctrl+C' - for exit.
Example of expression: 1 + 2.";
    println!("{}", welcome_str);
    loop {
        let mut text = String::new();
        print!("calc> ");
        io::stdout().flush().expect("Oops");
        match io::stdin().read_line(&mut text) {
            Ok(_) => {
                let trimmed = text.trim();
                text = trimmed.to_string();

                if text == "q" {
                    return
                }

                if text.len() == 0 {
                    continue
                }
            },
            Err(e) => {
                println!("{}", e)
            }
        };

        let lexer = Lexer::new(text);
        let mut interp = Interpreter::new(lexer);
        let result = interp.expr();
        println!("{}", result);
    }
}



