fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s);

    s.clear(); // makes the string equal to ""
               // after clearing word is invalid property.
}

// Write a function that takes a string of words separated by spaces and
// returns the first word it finds in that string. If the function doesn’t
// find a space in the string, the whole string must be one word, so the entire
// string should be returned.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
