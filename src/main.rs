use codecrafters_interpreter::{Lexer, Parser};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    if file_contents.is_empty() {
        println!("EOF  null");
        return;
    }

    let mut lexer = Lexer::new();
    lexer.scan_input(&file_contents);

    match command.as_str() {
        "tokenize" => {
            lexer.write_tokens_to_stream(&mut io::stdout(), &mut io::stderr());

            if lexer.has_errors {
                std::process::exit(65);
            }
        },

        "parse" => {
            if lexer.has_errors {
                std::process::exit(65);
            }

            let parser = Parser::new(lexer.tokens);
            let parsed_expression = parser.parse();
            parser.print_result(parsed_expression, &mut io::stdout(), &mut io::stderr());
        },

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
