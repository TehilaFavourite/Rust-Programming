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
    let person = String::from("Alice");
    println!("person is {person}");
    let genius = person;
    println!("genius is {genius}");
    // this does not implement the copy trait like the int example because the string type is a complex type.
    // complex types are stored on the heap and are not copied when assigned to a new variable.
    // the variable that was original owner will be invalidated and the new variable will be the new owner.
    // from the above, the genius variable is a new owner of the string "Alice".
    // the person variable is no longer valid here and is dropped.
    // the genius variable is now the owner of the string "Alice".
    // we can only have one owner of a value at a time.
    // we cannot use the person variable here because it is no longer valid.
    // we can only use the genius variable here because it is the owner of the string "Alice".

    // **************************************************
    // the drop function
    let girl = String::from("Doris");
    drop(girl);
    // println!("girl is {girl}");
    // from the above, the drop function is used to explicitly drop a variable.

    // **************************************************
    // the clone method
    let boy = String::from("Bob");
    let male = boy.clone();
    println!("boy is {boy}, male is {male}");
    // in the above, there is no transfer of ownership and no move.
    // there are two owners of the string "Bob", boy and male.

    // **************************************************
    // References and borrowing
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("heap value");
    let my_heap_reference = &my_heap_value;

    // **************************************************
    // the dereference operator
    let stack_value = 2;
    let integer_reference = &stack_value;
    println!("{}", *integer_reference);

    // **************************************************
    // String, &String, Str, &Str
    let ice_cream = "cookies and cream"; // this is an str. 
    // like the above, any piece id text you declare in double quotes is going to be encoded or embedded directly into the the binary executable

    // **************************************************
    // the copy trait with references
    let coffee = "nescafe";
    let breakfast = coffee;

    // **************************************************
    //Ownership and function parameters
    let apples = 6;
    print_my_value(apples);

    let oranges = String::from("oranges");
    print_my_val(oranges);

    let burger = String::from("burger");
    add_fries(burger);
}

// **************************************************
// function parameters

fn print_my_value(value: i32) {
    println!("you have {value} apples")
}

fn print_my_val(value: String) {
    println!("you have {value} as your fruit")
}

// **************************************************
// mutable parameters

fn add_fries(mut meal: String) {
    meal.push_str(" and fries");
    println!("{meal}")
}
