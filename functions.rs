fn main() {
    let countdown = 10;
    let count_closure = |num| {
        let mut c = 0;
        while num - c != 0 {
            println!("{}", num - c);
            c += 1;
        }
    };
    count_closure(countdown);
    let x = five();

    let y = {
        let x = if x > 0 { x*199 } else { -x };
        let y = x + 5;
        y + 1
    };

    another_function(x, y);
    println!("Double 4 if even: {}", double_even_number(4));
    println!("Double 5 if even: {}", double_even_number(5));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    6-1
}

fn double_even_number(x: i32) -> i32 {
    if x%2 == 0 {
        return x*2
    } else {
        x
    }
}
