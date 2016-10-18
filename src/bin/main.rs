extern crate clap;
extern crate gate;
extern crate rustyline;

use std::{fs, io};
use std::io::Read;

fn main() {
    let matches = clap::App::new("gate")
        .version("0.1.0")
        .about("A simple programming language")
        .arg(clap::Arg::with_name("interactive")
            .short("i")
            .long("interactive"))
        .arg(clap::Arg::with_name("INPUT").help("An optional file to run"))
        .get_matches();

    let mut program = gate::Program::new();
    let mut has_run = false;

    if let Some(input) = matches.value_of("INPUT") {
        run_file(&mut program, input);
        has_run = true;
    }

    if matches.is_present("interactive") {
        run_interactive(&mut program);
        has_run = true;
    }

    if !has_run {
        run_stdin(&mut program);
    }
}

fn run_interactive(program: &mut gate::Program) {
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
                        }
                        Err(gate::ParseError::ScanError(gate::TokenError::IncompleteString)) => {
                            needs_more_input = true;
                            break;
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            continue 'outer;
                        }
                    }
                }
            }

            if !needs_more_input {
                rl.add_history_entry(&line);

                let mut last_result = gate::Data::Nil;
                for expr in exprs {
                    last_result = match expr.eval(program) {
                        Ok(d) => d,
                        Err(e) => {
                            println!("error: {}", e);
                            continue 'outer;
                        }
                    };
                }
                println!("{:?}", last_result);
                continue 'outer;
            } else {
                line.push('\n');
                match rl.readline(">> ") {
                    Ok(l) => line.push_str(&l),
                    Err(_) => break 'outer,
                }
            }
        }
    }
}

fn run(program: &mut gate::Program, input: String) {
    let parser = gate::Parser::new(&input);
    for expr in parser {
        match expr.unwrap().eval(program) {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

fn run_file(program: &mut gate::Program, filename: &str) {
    let mut input_file = fs::File::open(filename).expect("can't open file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    run(program, input);
}

fn run_stdin(program: &mut gate::Program) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    run(program, input);
}
