mod set_1;

use crate::set_1::problem_1::main as problem_1;
use crate::set_1::problem_2::main as problem_2;
use crate::set_1::problem_3::main as problem_3;

mod byte_tools;

fn main() {
    println!("p1");
    problem_1();
    println!("p2");
    problem_2();
    println!("p3");
    problem_3();
}
