fn main() {
    let s = "hello"; // String literal
    let mut s1 = String::from("hello"); // has capacity, length and ptr properties
    s1.push_str(", world");
    println!("{s} world");
    println!("{}", s1);

    // The move. Happens when we assign heap based value.
    // As shallow copy.
    let s2 = s1;
    // println!("{}", s1); // s1 isn't available the reference binds with s2.

    // To make a deeply copy by .clone
    let s3 = String::from("hello");
    let s4 = s3.clone();

    takes_ownership(s3); // Produces an error, cause s3 was dropped in takes_ownership;
    println!("s3: {s3}, s4:{s4}");

    let s5 = gives_ownership();
    let s6 = takes_and_gives_ownership(s5);

    // An example of getting property of heap type variable value with returns ownership.

    let (s7, len) = calculate_length(s6);
    println!("{s7} has length: {len}");
}

fn calculate_length(some_string: String) -> String {
    let length = s.len();
    (some_string, length)
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership");
    println!(some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!(some_string);
    some_string
}
