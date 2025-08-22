fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert}, there will be a concert");
    println!("{is_event}, there will be a concert");

    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi} for dinner");
    println!("{dinner} for dinner");


    let sushi = String::from("Salmon");
    let dinner = sushi;
    // println!("{sushi} for dinner");
    println!("{dinner} for dinner");
    eat_meal(dinner);
    // println!("sushi: {}", dinner); // ❌ error: sushi was moved

    let mut dinner = String::from("Salmon");
    new_eat_meal(&mut dinner);

    println!("After eat_meal, sushi is: '{}'", dinner); // prints empty string

    


    

}

fn eat_meal(mut meal: String) {
    meal.clear();
    println!("Inside eat_meal, meal is now: '{}'", meal);

    //     new_dinner owns "Salmon" in main.

    // Calling eat_meal(new_dinner) transfers ownership to the parameter meal.

    // Inside eat_meal, meal.clear() empties the string.

    // When eat_meal ends, meal (the empty string) is dropped.

    // new_dinner in main is no longer valid — ownership was moved.
//     Ownership was moved from new_dinner in main → to the parameter meal in eat_meal.
// When the function ends, meal goes out of scope and Rust drops it, freeing the memory.
}

fn new_eat_meal(meal: &mut String) {
    meal.clear();
    println!("Inside eat_meal, meal is now: '{}'", meal);

//     Ownership of "Salmon" stays with another_dinner.

// eat_meal only borrows it mutably, clears it, then gives control back.

// main can still use another_dinner (now empty) after the function call.
}


/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.
 
Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.
 
The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.
 
In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.
 
Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/