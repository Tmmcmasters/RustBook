use std::string;

fn main() {
    // This program is safe to execute.
    let x = true;
    read(x);
    // This program is not safe to execute because the `y` variable is not initialized and since this checks at compile time, it is not safe to execute. Rust tries to avoid any sort of undefined behaviors by checking at compile time rather than at runtime like javascript.
    // read(y);
    // let y = false;

    //Safety is the absence of undefined behaviors. Ownership is about safety.

    // Memory is the space where data is stored during the execution of a program. These are many ways to think about memory.

    // A frame is a mappign from variables to values within a sigle scope, such as a function.

    transfer_data_without_copying();

    explaining_allocation_and_freeing();

    let a = simple_ownership();
    println!("{}", a);

    let first = String::from("this");
    let full = create_moves_and_mutates(first);
    println!("{}", full);

    //Can't use a variable after it has been moved
    use_after_move();

    use_after_move_with_clone();
}

fn use_after_move_with_clone() {
    let first = String::from("this");
    let full = no_move_and_mutates(first.clone());
    println!(
        "First: '{}' is now a different value than full since I cloned it into full: '{}'",
        first, full
    ); //Now I can print this value first since it was cloned into full. However, now they are different values
    println!("This is known as a deep copy")
}

fn no_move_and_mutates(mut name: String) -> String {
    name.push_str(" does not move and mutates");
    name
}

fn use_after_move() {
    let first = String::from("this");
    let full = create_moves_and_mutates(first);
    // println!("{}", first); //borrower of moved value: 'first' value borrowed here after move
    println!("{}", full);
}

fn create_moves_and_mutates(mut name: String) -> String {
    name.push_str(" moves and mutates");

    // Remember this simply returns name since this is an expression that returns a string, not a statement
    name
}

fn simple_ownership() -> string::String {
    let a = Box::new("hello world ownership");

    let b = a;
    return format!(
        "a origninally owned the box hello, but now b owns the value: {}",
        b
    );
}

// Rather than saying that a_box is a pointer to a number on the heap, we can just say that a_box owns the number on the heap. This is the idea of ownership and how things are transfered.
fn explaining_allocation_and_freeing() {
    let a_number = 4; // a number on the stack

    // There is just a_number on the stack before calling the make_and_drop function

    //During the call of the make_and_drop function, there is a pointer on the stack to the number on the heap
    make_and_drop(); // a number on the heap

    // After the execution of the make_and_drop function the a_box pointer to the number on the heap is dropped and no longer valid.
    //ALL THAT IS LEFT is a_number on the stack.
    println!("{:?}", a_number);
}

fn make_and_drop() {
    let a_box = Box::new(5); // a box is a pointer to 5 on the heap
    println!("{:?}", a_box);
}

fn transfer_data_without_copying() {
    // a is a pointer to an array on the heap of 100 zeroes
    // A pointer is just a reference to a location in memory
    let a = Box::new([0; 100]);

    // b is is now replacing a as the pointer to the heap of 100 zeros
    // a is no longer valid
    let b = a;

    // c is now a pointer to b on the stack
    // b is no longer valid
    let c = b;

    // if you try to println! like seen below, you will get a borrower moved error
    // println!("{} {} {}", a[0], b[0], c[0]); // borrow of moved value: 'a' value borrowed here after move

    // So you can just use c as you would any other pointer
    println!("{:?}", c.len());

    // c is now the only valid pointer to the array on the heap
    // when c goes out of scope, the array on the heap is dropped

    return;
}

fn read(y: bool) {
    if y {
        println!("y is true");
    }
}
