// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

fn main() {
    // let  x = 5; Compile error if we don't set the variable as mutable.
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    // Shadowing
    println!("Shadowing");
    let spaces = "    ";
    println!("spaces is {spaces}");
    let spaces = spaces.len();
    println!("spaces is {spaces}");

    // Error not allowed to mutate a variable’s type
    // let mut spaces = "   ";
    //spaces = spaces.len();

    // Tuples
    let tup = (500, 5.3, 2);
    let (a, b, c) = tup;
    println!("a: {a} b: {b} c: {c}");
}
