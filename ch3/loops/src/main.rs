fn main() {
    // The loop keyword executes a function over and over until the end of time;
    // in this case, it will print "Hello, world!" 3 times.
    let mut x = 0;
    // If you just have the printLn word in the loop body, it will print "Hello, world!" forever, but with the break keyword or an expression that will evaluate correectly then it will run right until the end.
    loop {
        x += 1;
        println!("x = {}", x);
        if x == 300 {
            println!("Bye Bye!");
            break;
        }
    }

    return_values_from_loops();
    labeled_loops();
    conditional_while();
    collection_looping();
    better_collection_looping();
    for_range_looping();

    // This loop will allow the program to run forever until the break keyword is called so you can run the .exe file to test it out.
    // loop {}
}

fn return_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labeled_loops() {
    //Count is initialized to zero
    let mut count = 0;

    // Here is the labeled loop count up
    'counting_up: loop {
        println!("count = {count}");
        //Remaining is initialized with 10
        let mut remaining: i32 = 10;

        // Here is the nested loop
        loop {
            println!("remaining = {remaining}");
            //if remaining is equl to 9, break the NESTED LOOP
            if remaining == 9 {
                break;
            }
            //if remaining is equl to 2, break the LABELED LOOP which will break everything
            if count == 2 {
                break 'counting_up;
            }
            //Decrement remaining
            remaining -= 1;
        }

        //Increment count
        println!("Incrementing outer count {}", count);
        count += 1;
    }
}

fn conditional_while() {
    println!("Starting the while loop");
    let mut number = 0;

    while number < 5 {
        println!("{number}");

        number += 1;
    }

    println!("All done with the wile loop");
}

fn collection_looping() {
    println!("Starting the collection loops");
    //A simple use of iterating over this collection of integar 32s. This is an array of 5 integar32s

    // This will count up from 0 to 4 and print the value of the array's index;
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1
    }
}

fn better_collection_looping() {
    //Starting the better collection looping.

    // This is a vastly more concise way of iterating over an array as it removes the more error prone way of iterating in the while loop for example if the array is a different size.

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_looping() {
    println!("Starting the range looping");

    // This will count down from 4 to 0 as the .rev reverses the order of the array
    for number in (1..5).rev() {
        println!("{number}");
    }

    println!("Finished the range looping");
}
