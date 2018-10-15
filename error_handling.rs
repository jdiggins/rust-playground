/* John Diggins
 * 10/15/2018
 * Chapter 9 Error Handling
 * https://doc.rust-lang.org/book/second-edition/
 *
 * Notes:
 *
 * Rust provides two types of errors:
 * Recoverable - error can be reported and operation can be tried again
 *   ex. Error finding a file
 * Result<T, E>

 * Unrecoverable - not recoverable.. symptoms of bugs
 *   ex. trying to access location beyond the edge of an array
 * panic!
 * 
 * When panic! macro executs, program prints failure message, unwinds stack, quits
 * 
 * Add this to Cargo.toml to immediately abort on panic!
 *   [profile.release]
 *   panic = 'abort'
 * 
 */

 

fn main() {

    /* using panic! backtrace */
    // let v = vec![1, 2, 3];
    // v[99];

    /* call panic */
    // panic!("crash and burn");

    /* Return Enum:
     *   enum Result<T, E> {
     *      Ok(T),
     *      Err(E),
     *   }
     */
    use std::fs::File;

    /* File::open is a Return enum ^^ */
    // let f = File::open("hello.txt");

    /* Check the return enum, if good set f to file, else panic! */
    // let f = match f{
    //    Ok(file) => file,
    //    Err(error) => {
    //        panic!("there was a problem opening the file: {:?}", error)
    //    },
    // };

    use std::io::ErrorKind;

    /* Match on different errors */

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    /* Shortcuts for Panic on Error: unwrap and expect */

    //let f = File::open("hellounwrap.txt").unwrap();

    //let f = File::open("helloexpect.txt").expect("Failed to open helloexpect.txt");

    //////////////////////////
    // 9.3 panic! or not to panic!
    //

    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess {
                value
            }
        }
        pub fn value(&self) -> u32 {
            self.value
        }
    }

}


/* Propagating Errors: returning an error to the caller */
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/* Shortcut for propagating errors: the ? operator
 * The ? operator can only be used in functions that return Result
 */

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn short_short_read_username_from_file() -> Result<String, io:Error> {
    let mut s = String::new();

    File::open("file.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
