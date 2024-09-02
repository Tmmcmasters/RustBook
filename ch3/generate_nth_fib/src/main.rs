fn main() {
    println!("Starting the fibonacci cli...");

    for i in 0..10 {
        println!("{}", fibonacci(i));
    }

    println!(
        "Calling the place of the fibonacci sequence at {}:",
        fibonacci(21)
    );
}

fn fibonacci(n: i32) -> i32 {
    // This function is a recursive function, which means it calls itself in its own definition
    // It takes an i32 and returns the nth number in the fibonacci sequence
    // The fibonacci sequence is a series of numbers where each number is the sum of the last
    // two numbers in the sequence. The sequence starts with 1 and 1. So the sequence is
    // 1, 1, 2, 3, 5, 8, 13, and so on.
    // This function works by checking if the number is 2 or less. If it is, it returns that
    // number. Otherwise it calls itself with n - 1 and n - 2 and adds the two results together.
    // Calling itself with n - 1 and n - 2 is a shorthand way of writing the following
    // pseudocode:
    //
    // if n is 2 or less
    //   return n
    // else:
    //   set a to the fibonacci of n - 1
    //   set b to the fibonacci of n - 2
    //   return a + b
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
