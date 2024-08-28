fn main() {
    // Rust has four primary scalar types
    // - integers
    // - floating-point numbers
    // - booleans
    // - characters

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // The tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length.

    let mut tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    // The :? in the println! macro is a placeholder for a type that implements std::fmt::Debug. This is a trait that provides a way to format values in a programmer-readable form. The Debug trait is supported by most of the primitive types and compound types in Rust's standard library.
    println!("{tup:?}");

    println!("The value of y is: {}; x is: {}; z is: {}", y, x, z);

    tup.0 = 999;
    println!("{}", tup.0);

    // The array type is a data structure that holds a fixed-size collection of elements of the same type. Arrays are useful when you want your data to be homogenous.Arrays are useful when you want your data allocated on the stack rather than the heap

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{:?}", months);

    // This is an array of 5 elements, all of which are equal to 3.
    let a = [3; 5];

    println!("{:?}", a[0]);

    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
}
