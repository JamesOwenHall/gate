extern crate clap;
extern crate gate;
extern crate rustyline;

use std::io;
use std::io::Read;

fn main() {
    let matches = clap::App::new("gate")
        .version("0.1.0")
        .author("James Hall")
        .about("A simple programming language.")
        .arg(clap::Arg::with_name("interactive")
            .short("i")
            .long("interactive"))
        .get_matches();

    if matches.is_present("interactive") {
        run_interactive();
    } else {
        run_stdin();
    }
}

fn run_interactive() {
    let mut program = gate::Program::new();
    let mut rl = rustyline::Editor::new();

    'outer: loop {
        let mut line = match rl.readline("> ") {
            Ok(l) => l,
            Err(_) => break 'outer,
        };

        loop {
            let mut needs_more_input = false;
            let mut exprs = vec![];

            {
                let parser = gate::Parser::new(&line);
                for expr_res in parser {
                    match expr_res {
                        Ok(e) => exprs.push(e),
                        Err(gate::ParseError::UnexpectedEOF) => {
                            needs_more_input = true;
                            break;
                        },
                        Err(e) => {
                            println!("{:?}", e);
                            continue 'outer;
                        },
                    }
                }
            }

            if !needs_more_input {
                rl.add_history_entry(&line);

                let mut last_result = gate::Data::Nil;
                for expr in exprs {
                    last_result = expr.eval(&mut program);
                }
                println!("{:?}", last_result);
                continue 'outer;
            } else {
                match rl.readline(">> ") {
                    Ok(l) => line.push_str(&l),
                    Err(_) => break 'outer,
                }
            }
        }
    }
}

fn run_stdin() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parser = gate::Parser::new(&input);
    let mut program = gate::Program::new();
    
    for expr in parser {
        expr.unwrap().eval(&mut program);
    }
}
