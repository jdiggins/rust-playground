/* John Diggins
 * 10/9/18
 * Rust loops n loops
 */

fn main() {
    let a = [1, 2, 3, 4, 5];

    

    for e in a.iter() {
        println!("the value is: {}", e);
    }

    let mut x = 2;
    loop {
        println!("{}", x);
        if x > 10 { break; }
        x += 1;
    }

    for n in (2..11).rev() {
        println!("{}", n);
    }

    let mut a = 1;
    let mut b = a;

    loop {
        println!("{}", a);
        let c = a;
        a = b;
        b = c+b;
        
        if a > 10000 { break; }
    }
}