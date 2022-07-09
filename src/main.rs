use ariadne::{Label, Report, ReportKind, Source};
use combine::ParseError;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use thorn::parser::parser::parse;
use thorn::typechecker::checker::Typechecker;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let typechecker = Typechecker::new();
                rl.add_history_entry(line.as_str());
                let parser_result = &parse(&line).map(|x| x.0);

                if let Ok(parsed) = parser_result {
                    let checked =
                        typechecker.check(&thorn::typechecker::types::LangType::Any, parsed);

                    println!("{parsed:?}");

                    match checked {
                        Ok(_) => println!("Typechecking passed"),
                        Err(ref e) => {
                            Report::build(ReportKind::Error, (), 0)
                                .with_message(e.to_short_error().to_string())
                                .with_label(Label::new(0..1))
                                .with_label(Label::new(0..1).with_message(format!("{e}")))
                                .finish()
                                .print(Source::from(&line))
                                .unwrap();
                        }
                    }
                } else {
                    let err = match parser_result {
                        Err(e) => e,
                        _ => unreachable!(),
                    };
                    Report::build(ReportKind::Error, (), 0)
                        .with_message({
                            let err_vec = format!("{}", err)
                                .split('\n')
                                .map(|s| s.to_string())
                                .skip(1)
                                .collect::<Vec<String>>();
                            err_vec[0].to_owned() + "\n" + &err_vec[1]
                        })
                        .with_label(
                            Label::new(
                                err.position().line as usize..err.position().column as usize,
                            )
                            .with_message(err.errors.first().unwrap()),
                        )
                        .finish()
                        .print(Source::from(&line))
                        .unwrap();
                }
            }
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
