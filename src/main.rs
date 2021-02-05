fn main() {
    // to allow a variable to be mutable you assign it with `mut`
    let mut x = 5; // by default ints are signed
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    x = x + 1;
    println!("The value of x is {}", x);
    // shadowing: we reassign `spaces` by initializing a new variable with the same name
    let spaces = "  ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
    // wrapping usigned numbers
    let y: u8 = 1; // 1 as an unsigned 8 bit integer
    // You can also assign it like this.
    // let mut y =1u8;
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
    let my_tup: (&str, bool, i32, f64) = ("my tuple", true, 100, 11.3);
    let (name, is_tuple, my_int, my_float) = my_tup;
    println!("name: {}, is_tuple: {}, my_int: {}, my_float: {}", name, is_tuple, my_int, my_float);
    let the_truth = my_tup.1;
    println!("the truth is {}", the_truth);
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // Prints the second months
    println!("{}", months[1]);
    print!("\nLoop through all the months:\n\n");
    for (i, month) in months.iter().enumerate() {
        println!("{}: {}", i + 1, month);
    }
}
