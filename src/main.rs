fn main() {
    // to allow a variable to be mutable you assign it with `mut`
    let mut x = 5; // by default ints are signed
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    x = x + 1;
    println!("The value of x is {}", x);
    let spaces = "  ";
    // shadowing: we can reassign `spaces` by initializing a new variable with the same name
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
    // wrapping unsigned numbers
    let y: u8 = 1; // 1 as an unsigned 8 bit integer
    // You can also assign it like this.
    // let mut y = 1u8;
    // you need to explicitly subtract with wrapping if you're going to do stuff like this.
    println!("y = {}", y.wrapping_sub(2)); // 255
    /*
    The following line will not compile because 255 is the highest you can have with 8 bits
    let too_high: u8 = 256;

    error: literal out of range for `u8`
    --> src/main.rs:4:24
    |
    4 |     let too_high: u8 = 256;
    |                        ^^^
    |
    = note: `#[deny(overflowing_literals)]` on by default
    = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
    println!("{}", too_high);
    */

    // this is a tuple
    let my_tup: (&str, bool, i32, f64) = ("my tuple", true, 100, 11.3);
    let (name, is_tuple, my_int, my_float) = my_tup;
    println!("name: {}, is_tuple: {}, my_int: {}, my_float: {}", name, is_tuple, my_int, my_float);
    // you can also assign variables like this
    let the_truth = my_tup.1; // true (bool)
    println!("the truth is {}", the_truth);

    // this is an array
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // Prints the second months
    println!("{}", months[1]);
    print!("\nLoop through all the months:\n\n");
    // if we didn't want the index we would just do `for month in months.iter(){println!("{}", month)}
    for (i, month) in months.iter().enumerate() {
        println!("{}: {}", i + 1, month);
    }

    print!("\n");
    let y = {
        // this is x is scoped, the x outside is still 7
        let x = 3;
        // no semicolon here - this is an expression, not a statement.
        x + 1
    };
    println!("The value of y is {}", y);
    // call new_function
    print_two_ints(12, 25);
    println!("The value of x is still {}", x);
    let added = add_two_ints(12, 25);
    println!("12 + 25 = {}", added);
}

fn print_two_ints(x: i32, y: i32) {
    println!("The value of x in print_two_ints is {}", x);
    println!("The value of y in print_two_ints is {}", y);
}

fn add_two_ints(x: i32, y: i32) -> i32 {
// no semicolon
    x + y
}