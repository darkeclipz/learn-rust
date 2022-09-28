// Used to get rid of annoying warnings, since this is a tutorial anywayss..
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world 2!");
    variables();
    shadowing();
    data_types();
    functions();
}

fn variables() {

    // Immutable variables.
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // cannot assignt twice to immutable variable
    println!("The value of x is: {x}");

    // Mutable variables.
    let mut y = 10;
    println!("The value of x is: {y}");
    y = 20;
    println!("The value of x is: {y}");
    
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    // you can declare a new variable with the same name
    // as a previous variable, this is called shadowing.

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2; // shadows x = x + 1
        println!("The value of x in the inner scope is: {x}");

        // scope ends so inner shadowing ends.
    }

    println!("The value of x is: {x}");

    // shadowing can also be used to change the data type,  however
    // this is only possible if the variable is immutable.
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    // a scalar type represents a single value, there are four primary
    // scalar types: integers, floating-point numbers, Booleans and chars.

    fn integer_types() {
        let int8: i8 = -128; // 8-bit    i8, u8 (u = unsigned)
        println!("The value of int8 is: {int8}");
        // ...

        // Length	Signed	Unsigned
        // 8-bit	i8	u8
        // 16-bit	i16	u16
        // 32-bit	i32	u32
        // 64-bit	i64	u64
        // 128-bit	i128	u128
        // arch	isize	usize

        // isize and usize depends on the architecture of the system.
        let unsigned_arch_int: usize = 9_999_999;
        println!("The value of unsigned_arch_int is: {unsigned_arch_int}");
    }

    fn integer_literals() {
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';

        // in debug mode, Rust panics if there is an overflow.
        // in release mode, Rust wraps if there is an overflow.
    }

    fn floating_point_types() {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
        let difference = x - y;
        let floored = 2 / 3; // results in 0.
        let remainder = 43 % 5;
    }

    fn character_type() {
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';

        // let x_😻 = 5; // identifiers cannot contain emoji.
    }

    fn tuple_type() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup; // deconstruct
        println!("The value of x is: {}", tup.0); // 0-indexed
        println!("The value of y is: {y}");
    }

    fn array_type() {
        let a = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July",
                                  "August", "September", "October", "November", "December"];

        let b = [10; 10]; // array of [10, 10, 10, ..., 10]
                                     // [value; n-times]

        let c = b[2];
        println!("The value of c is: {c}");

        // Rust panics if you index out of bounds.
    }

    integer_types();
    integer_literals();
    floating_point_types();
    character_type();
    tuple_type();
    array_type();

}

fn functions() {

    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");

}
