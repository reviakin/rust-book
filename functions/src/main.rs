// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn main() {
    println!("Hello, world!");
    another_function(3000000, 'a');
    another_another_function();
    let x = five();
    println!("{x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("the value is {x} {unit_label}");
}

fn another_another_function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
