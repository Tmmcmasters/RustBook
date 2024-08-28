fn main() {
    //Simple if statement here;
    let number = 2;

    if number > 5 {
        println!("number is greater than 5");
    } else {
        println!("number is less than 5");
        println!("number is {}", number);
    }

    // You cannot do something like this:
    // if number {
    //     print!("number is {}", number);
    // }

    let number: i32 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in let statements

    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    //The same code with  a &str in the second expression will result a compile error as seen below
    // let number: i32 = if condition { 5 } else { "&str" };

    println!("The value of number is: {}", number);
}
