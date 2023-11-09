mod errors;
mod parsing;

use parsing::{scanner, parser, visitor};

struct Lox {
    has_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox { has_error: false }
    }

    fn run_file(&self, path: &String) {
        let input = File::open(path).unwrap();
        let buffer = BufReader::new(input);
        let lines: Vec<String> = buffer.lines().map(|x| x.unwrap()).collect();
        println!("running file {}", path);
        self.run(&lines.join("\n"));
    }

    fn run_prompt(&self) {
        print!("> ");
        _ = stdout().flush();
        let mut input: String;
        for line in io::stdin().lines() {
            input = line.unwrap();
            if input.to_lowercase() == ".exit" {
                break;
            }
            self.run(&input);
            print!("> ");
            _ = stdout().flush();
        }
    }

    fn run(&self, input: &String){
        let mut scanner = scanner::Scanner::new(input.to_owned());
        let tokens_result = scanner.scan_tokens(); 
        match &tokens_result {
            Err(_) => {return}
            Ok(_) => {}
        }

        let tokens = tokens_result.unwrap();
        // println!("{:?}", tokens);
        
        let mut parser = parser::Parser::new(tokens);
        let expr = parser.parse();
        if expr.is_none() {
            eprintln!("parsing err!");
            return;
        }

        // let printer = visitor::Printer::new();
        // let str_tree = printer.print(expr.unwrap());
        // println!("{}", str_tree.ok().unwrap());

        let mut interpreter = visitor::Interpreter::new();
        let res = interpreter.evaluate(expr.unwrap());
        if res.is_err(){
            let e = res.err().unwrap();
            println!("{:?}", e);
        }
        else {
            println!("{:?}", res.ok().unwrap().val);
        }
    }
}

use std::{
    env,
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
