use hoya::interpreter::interpret::Interpreter;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    rl.load_history("history.txt")?;

    let interpreter = Interpreter::default();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => interpreter.interpret(&line),
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
