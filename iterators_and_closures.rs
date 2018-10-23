/* John Diggins
 * 10/23/18
 * https://doc.rust-lang.org/book/2018-edition/
 * Chapter 13 - Functional Language Features: Iterators and Closures
 *
 * Notes:
 *
 * Iterators implement a trait named Iterator
 *
 *
 */

use std::thread;
use std::time::Duration;

/* A function and equivalent closures
 * fn  add_one_v1   (x: u32) -> u32 { x + 1};
 * let add_one_v2 = |x: 32|  -> u32 { x + 1 };
 * let add_one_v3 = |x|             { x + 1 };
 * let add_one_v4 = |x|               x + 1  ;
 */
/* Closure! */
/*
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};*/

struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
/* simulates a function that takes 2 seconds to run */
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    /* Closure! */
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

/* function version 1 before added closure */
fn generate_workout_without_closure(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

struct Counter {
    count: u32,
}

/* implement iterator */
impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    /* Closure can access variables from  the scope they are defined */
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    /* Iterators */

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1_iter2 = v1.iter();
    for val in v1_iter2.rev() {
        println!("Got: {}", val);
    }

    let v2: Vec<_> = v1.iter().map(|x| x  + 1).collect();

    let v2_iter = v2.iter();

    for val in v2_iter {
        println!("Got: {}", val);
    }
}