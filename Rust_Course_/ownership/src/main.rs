fn main() {

    //scope and ownersip
    let age = 33; // age here is the owner of the value 33
    {
        let is_handsome = true;
    } // is_handsome goes out of scope here and is dropped.
    // age variable exists here and is still valid.
    println!("age is {age}");
    // from the above, what is put in the stack first is the age variable, and then the is_handsome variable.
    // **************************************************
    // the copy trait
    let time = 2025;
    let year = time;
    println!("time is {time}, year is {year}");
    // from the above, the right hand of the equal sign is evaluated first.
    // integers are simple values and are copies, not moved.
    // they implement the copy trait, which means they are copied when assigned to a new variable.
    // we now have two owners of the value 2025, time and year.
    
    // **************************************************
    // string type stored on the heap
    let food = "pasta"; // this is a string literal, which is a string stored in the binary.

    let text = String::new(); // new is a function that lives within the string namespace.

    let candy = String::from("sweet"); // from is a function that lives within the string namespace.

    // **************************************************
    // the push_str method on a string type
    let mut name = String::from("John");
    println!("name is {name}");
    name.push_str(" Doe");
    println!("name is {name}");
    // from the above, the push_str method is used to append a string to the end of the string.
    // the push_str method takes a string slice as an argument.
    // the push_str method does not take ownership of the string slice.
    // the push_str method does not return a value.
    

    // **************************************************
    // moves and ownership
    




}
