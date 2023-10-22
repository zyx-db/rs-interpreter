use std::{
    io::{BufReader, BufRead, self, stdout, Write},
    fs::File,
    env,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        return;
    } else if args.len() == 2{
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String){
    let input = File::open(path).unwrap();
    let buffer = BufReader::new(input);
    let lines : Vec<String> = buffer
        .lines()
        .map(|x| x.unwrap())
        .collect();
    println!("running file {}", path);
}

fn run_prompt(){
    print!("> ");
    _ = stdout().flush();
    let mut input: String;
    for line in io::stdin().lines() {
        input = line.unwrap();
        if input.to_lowercase() == "exit" {
            break;
        }
        println!("you entered {}", input);
        print!("> ");
        _ = stdout().flush();
    }
}
