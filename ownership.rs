/* John Diggins
 * 10/10/18
 * Ownership! References, borrowing, scope, slice
 */

fn main() {
    let mut s1 = String::from("hello");

    s1.push_str(", world");

    println!("{}", s1);

    let s2 = s1.clone();
    println!("{}", s2);
    let mut s2 = s1;

    /* If &mut is not used, s2 in main will lose ownership */
    change(&mut s2);

    println!("{}", s2);

    /* Slice */
    let word = first_word(&s2);

    println!("{}", word);

    let some_words = "some words in a string";

    println!("{}", first_word(some_words));
}

fn change(some_string: &mut String) {
    some_string.push_str("!");
}

/* Slice 
 * &String can be sent to type &str
 */

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}