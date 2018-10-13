/* John Diggins
 * 10/12/18
 * FizzBuzz
 * First solution to fizzbuzz problem
 */
fn main() {
    
    for n in 0..21 {
        let mut print = true;
        if n % 3 == 0 {
            print!("{}", String::from("Fizz"));
            print = false;
        }
        if n % 5 == 0 {
            print!("{}", String::from("Buzz"));
            print = false;
        }
        if n % 7 == 0 {
            print!("{}", String::from("Baxx"));
            print = false;
        }
        if print {
            print!("{}", n);
        }
        println!("");

    }
}