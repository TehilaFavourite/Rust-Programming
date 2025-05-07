fn main() {
    // declare variables with and without mut
    let x = 5;  // immutable variable
    let mut y = 10; // mutable variable

    println!("immutable x is: {}", x);
    println!("mutable y is: {}", y);

    // reassign value to mutable variable
    y = 20;

    println!("updated mutable y is: {}", y);
}


