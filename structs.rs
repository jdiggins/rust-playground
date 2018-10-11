/* John Diggins
 * 10/11/18
 * Structs!
 */
 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



/* User building function */
fn build_user(email: String, username: String) -> User {
    User {
        email, /* Short for email: email, whens when var and field same name */
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)] /* needed to use println! on struct */
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {

    let user1 = User {
        email: String::from("user1@user1.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    /* Create new user with some info from user1
    * Could be useful when updating user email / username */
    let user2 = User {
        email: String::from("newemail@email.com"),
        username: String::from("newusername"),
        ..user1
    };

    let user3 = build_user(String::from("user3@email.com"), String::from("user3!!"));

    println!("User1 username: {} \nUser2 username: {} \nUser3 username: {}",
        user1.username, user2.username, user3.username);

    /* Tuple structs without names */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = (30, 50);
    let rect2 = Rectangle { width: 30, height: 50 };
    println! (
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    /* This will print:
     * rect 2 is Rectangle { width: 30, height: 50}
     */
    println!("rect2 is {:?}", rect2);
   
    /* This will print:
     * rect 2 is Redtangle {
     *   width: 30,
     *   height: 50
     * }
     */
    println!("rect2 is {:#?}", rect2);
    

    /* Called using method (internal function) */
     println!("The area of rect2 is {}.",
        rect2.area());

    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 50, height: 50 };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
