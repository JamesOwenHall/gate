extern crate clap;
extern crate gate;

use std::io;
use std::io::Read;

fn main() {
    clap::App::new("gate")
        .version("0.1.0")
        .author("James Hall")
        .about("A simple programming language.")
        .get_matches();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parser = gate::Parser::new(&input);
    let mut program = gate::Program::new();
    
    for expr in parser {
        expr.unwrap().eval(&mut program);
    }
}
