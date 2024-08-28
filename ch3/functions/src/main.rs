//This also includes stuff about comments

/*Here is a multi line comment


bruh
*/

fn main() {
    println!("Here is the main function ");

    another_function(5);
    println!("Value of x is {}", add(33, 33));
    expression();
    println!("Value of five is {}", five());
    println!("Value of five + 1 is {}", add_one(5));
    println!(
        "Here is a really cool one that calculates to {}",
        cool_syntax()
    );
}

fn another_function(x: i32) {
    println!("Here is the another function.");
    println!("Value of x is {}", x);
}

//The -> is the return type
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

//The return type can be as simple as this
fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn expression() {
    //This is an expression.
    let y = {
        let x = 3;
        // Do not add a semi colon or that will be a statement like the thing above which will not return a value
        x + 1
    };
    println!("Value of y in the expression function is {}", y);
}

fn f(x: i32) -> i32 {
    x + 1
}

fn cool_syntax() -> i32 {
    // This syntax is a function call to `f` with an expression as its argument,
    // which is a block that evaluates to the value of `y + 1`, which is then passed
    // to `f` and its return value is returned from `cool_syntax`.
    f({
        let y = 1;
        y + 1
    })
}
