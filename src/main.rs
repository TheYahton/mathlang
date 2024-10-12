use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use mathlang::interpreter::interpret;
use mathlang::lexer::tokenize;
use mathlang::parser::ast;

fn meta(source: &String) {
    println!("{}", interpret(ast(tokenize(&source))));
}

fn main() {
    println!("Mathlang 0.0.1");
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mathlang FILE or mathlang -i \"1/0\"");
        return;
    }

    if args[1] == "-i" {
        if args.len() > 2 {
            meta(&args[2]);
            return;
        }
        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();
            let mut input_string = String::new();
            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            meta(&input_string);
        }
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        println!("No such file or directory.");
        return;
    }
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why),
        Ok(file) => file,
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("{}", why),
        Ok(_) => (),
    }

    meta(&text);
}
