/* John Diggins
 * 10/16/2018
 * Chapter 10 Generic Types, Traits, and Lifetimes
 * https://doc.rust-lang.org/book/second-edition/
 *
 * Notes:
 *
 * No runtime cost for using generics
 * 
 */



/* Original function, not generic 
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
*/

/* Generic function to find largest item in array
 * Generic comparing function cannot use < > without adding some traits!
 * Not all types can use < >, so we must define Partial Ord
 * Must also include teh Copy trait
 */
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/* Generics with struct */
struct Point<T> {
    x: T,
    y: T,
}
/* Generics in Method Definitions */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
/* Function only for type f32 */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point_Mixed<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point_Mixed<T, U> {
    fn mixup<V, W>(self, other: Point_Mixed<V, W>) -> Point_Mixed<T, W> {
        Point_Mixed {
            x: self.x,
            y: other.y,
        }
    }
}

/* Generic with Enums */
enum Result<T, E> {
    Ok(T), 
    Err(E),
}

/* Implementing a Trait on a Type */

pub trait Summary {
    /* if no default implementation, would look like this
    fn summarize(&self) -> String;
    */

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String{
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String{
        format!("@{}", self.username)
    }
}

/* Trait Bounds */
pub fn notift<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

/* Lifetime annotations 'a */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

fn main() {
    /* fn largest */
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /* struct Point */
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    /* Cannot mix types in generic <T, T> -> we need something like <T, U> */
    // let wont_work = Point { x: 5, y: 4.0 }
    /* The following are all valid with Point_Mixed<T, U> */
    let both_integer = Point_Mixed { x: 5, y: 10 };
    let both_float = Point_Mixed { x: 1.0, y: 4.0 };
    let integer_and_float = Point_Mixed { x: 5, y: 4.0 };

    /* struct generic function */
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    /* f32 distance_from_origin on Point<T>, method will only be available to f32 types */
    let f = Point { x: 3.0, y: 4.0 };
    println!("Distance from origin of point f: {}", f.distance_from_origin());

    /* mixup method */
    let p1 = Point_Mixed { x: 5, y: 10.4 };
    let p2 = Point_Mixed { x: "Hello", y: 'c' };
    
    let p3 = p1.mixup(p2); /* p3.x = p1.x, p3.y = p2.y */

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    /* Traits */
    let tweet = Tweet {
        username: String::from("super_tweet_user"),
        content: String::from("I make the superest tweets"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /* Call default trait implementation */
    let article = NewsArticle {
        headline: String::from("Sabers win the Stanley Cup Championship!"),
        location: String::from("Buffalo, NY, USA"),
        author:   String::from("Billy Buffalo"),
        content:  String::from("The Buffalo Sabers once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    /* Lifetime */
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

}