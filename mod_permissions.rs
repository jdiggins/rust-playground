/* John Diggins
 * 10/14/18
 * https://doc.rust-lang.org/book/second-edition/
 * Chapter 7
 */

/* Mod is module, functions can sit in modules to organize them
 * Modules can also be access from seperate files
 * pub = public
 * default = private
 */

////////////////////////
// 7.2
//
mod outermost{
    pub fn middle_function () {}

    fn middle_secret_function() {}
    pub mod inside { /* if this isn't public entire inside mod is private */
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
   // outermost::middle_secret_function();  // this will cause errors
    outermost::inside::inner_function(); 
   // outermost::inside::secret_function(); // this will cause errors
}

////////////////////////
// 7.3
//

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// use keyword!
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/* Multiple items can be brought into scope like so:
 * use TrafficLight::{Red, Yellow};
 */

 /* All names brought into scope at once: */
 use TrafficLight::*;


fn main() {
    a::series::of::nested_modules();

    nested_modules();

    let red = Red;
    let yelllow = Yellow;
    let green = Green;
}