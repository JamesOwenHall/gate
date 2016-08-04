extern crate gate;

use gate::{Parser, Program};

fn main() {
    let mut parser = Parser::new("false");
    let expr = parser.next().unwrap().unwrap();

    let mut p = Program::new();
    let res = expr.eval(&mut p);

    println!("{:?}", res);
}
