fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let y = 10;
    //If you try to reassign x, you'll get an error
    // x = 5;

    {
        //Shadowing
        let mut x = x + 10;
        x = x + 1;
        println!("The value of x in the inner scope is: {}", x);
    }

    //Shadowing
    let x = x + 5;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
}
