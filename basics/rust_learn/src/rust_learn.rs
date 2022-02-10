use learn_utils::math::{add, subtract};
use learn_utils::printer::print_text;

pub fn run() {
    let a: i8 = 5;
    let b: i8 = 6;
    let c: i8 = add(a, b);
    let d: i8 = subtract(a, b);
    
    println!("The add result: {}", c);
    println!("The subtract result: {}", d);
    println!("Together: {}", add(c, d));

    let text: String = String::from("hi");
    print_text(text);
}