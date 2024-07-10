use std::fs::File;
use std::io::Read;
use std::path::Path;

use mathlang::interpreter::interpret;
use mathlang::lexer::tokenize;
use mathlang::parser::ast;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
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
        println!("{}", interpret(ast(tokenize(&text))));
    } else {
        println!("Usage: mathlang FILE");
        /*
        println!("Mathlang 0.0.1");
        let mut input_string = String::new();
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {}", input_string);
        */
    }
}
