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
}
