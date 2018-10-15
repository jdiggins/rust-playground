/* John Diggins
 * 10/14/2018
 * Chapter 8 Common Collections
 * https://doc.rust-lang.org/book/second-edition/
 *
 * Notes:
 *
 * Common collections are stored on the heap, so amount of data
 * does not need to be known at compile time
 * 
 * This chapter discusses three collections:
 * Vectors, Strings, and Hash Maps
 */

fn main() {

    /////////////////////////
    // 7.1 Vectors
    // 

    /* Creating new empty vector to hold values of type i32 */
    let v: Vec<i32> = Vec::new();

    /* Rust can also infer type so it can be done like this */

    let mut v = Vec::new();
    v.push(1); // must add something so Rust can infer type

    /* macro vec! can also be used */
    let v = vec![1, 2, 3];

    /* remake as mute and add some values */

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    /* Dropping a vector drops its elements */
    {
        let drop_vec = vec![1, 2, 3, 4];
    } /* <--  drop_vec is dropped here */

    /* Read some elements of vectors */

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // let does_not_exist = &v[100]; <-- crashes program
    let does_not_exist = v.get(100); // returns None 

    /* Borrowing prevents pushing */

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); <-- this does not work! first must go out of scope 

    /* Iterate */
    let v = vec![100, 32, 25, 83];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 52, 83];
    for i in &mut v {
        *i += 50; // * dereferences i to get mutable value
    }

    /////////////////////////
    // 7.1 Strings
    // 
    // String is Collection, &str is core type
    // Rust also has CString, CStr, OsString, OsStr

    let mut s = String::new(); // empty string

    let data = "initial contents";

    let s = data.to_string(); // creates string from a string literal

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    /* Updating Strings */

    /* appending */

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 looses ownership!

    /* format! macro will not loose ownership */
    /* format! similar to println! */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); 

    /* Strings do NOT support indexing, it uses Slicing */

    let hello = "hello";
    let s = &hello[0..2];

    /* accessing chars in strings */
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    /////////////////////////
    // 7.3 Hash Maps!!!!!!
    //

    /* Creating a new hash map */
    use std::collections::HashMap; // use can be used here!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // needed to infer type
    scores.insert(String::from("Yellow"), 50);

    /* Zipping */

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /* Access values */

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /* Updating a Hash Map - Overwrite old values */

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    /* Only insert value if Key has no value */

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    /* Updating a value based on the Old value */

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    
}