/* John Diggins
 * 10/11/18
 * From chapter 6 doc.rust-lang.org/book/second-edition
 * Pattern Matching and Enums
 */

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move  { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/* function called on enum Message self
 * uses match to perform correct action
 */
impl Message {
    fn call(&self) {
        match *self {
            Message::Write(ref m) => println!("{}", m),
            Message::Move {x, y} => {},
            Message::Quit => {},
            Message::ChangeColor(_,_,_) => {},
        }

    }
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

////////////////////////////////////////////
// 6.2 match Control flow operator
//

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/* Matches are exhaustive
 * They must cover every possible case!
 * The following will not compile without "None => None"
 */
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let non = plus_one(None);

    ////////////////////////////
    // 6.3 if let
    //

    let some_u8_value = Some(3);

    /* This code can be rewritten with if let 
     * match some_u8_value {
     *    Some(3) => println!("three"),
     *    _ => (),
     * }
     */

    /* the if let */
    if let Some(3) = some_u8_value {
        println!("three");
    }




}
