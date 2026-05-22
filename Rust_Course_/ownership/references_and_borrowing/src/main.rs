fn main() {
    let current_meal = String::new();
    add_flour(current_meal);
    let current_meal = String::new();
    show_my_meal(current_meal);
    let current_meal = String::new();
    show_my_meal2(&current_meal);
    let mut current_meal = String::new();
    add_flour2(&mut current_meal);

    //****************************************** Multiple Immutable references */
    let car = String::from("red");
    let ref1 = &car;
    let ref2 = &car;
    println!("these are immutable references: {ref1} and {ref2} and {} ", &car);

    //****************************************** Mutable References Restrictions */
    let mut car = String::from("red");
    let ref1 = &mut car;
    let ref2 = &car; 
    // println!("{ref1} and {ref2} "); this will  give an error `immutable borrow occurs here`
    println!("{ref2} ");

    let mut car = String::from("red");
    let ref1 = &mut car;
    ref1.push_str(" and silver");
    let ref2 = &car; 
    println!("{ref2} ");

     //****************************************** Ownership with mutable and immutable reference */
    let coffee = String::from("Mocha");
    let a = &coffee;
    let b = a;
    println!("{a} and {b}");

    //****************************************** Ownership with arrays and tuples */
    let registrations = [true, false, true];
    let first = registrations[0];
    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("Solidity")];
    let first = languages[0].clone();
    let second = &languages[1];
    println!("{first} and {second} and {languages:?}");

    let registrations = (true, false, true);
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    let languages = (String::from("Rust"), String::from("Solidity"));
    let first = languages.0.clone();
    let second = &languages.1;
    println!("{first} and {second} and {languages:?}");


}

fn add_flour(mut meal: String) -> String {
    meal.push_str("add flour");
    meal
    // the problem here is that the meal here takes ownership of the string in the main function
    // this  is not a technical problem. it's an inconvenience.
}

fn show_my_meal(meal: String) {
    println!("meal steps: {meal}");
    //this is also same problem in add_flour
}

fn show_my_meal2(meal: &String) {
    println!("this is the second meal: {meal}");
    // meal here is no longer a string. It's a reference to a string
}

fn add_flour2(meal: &mut String) {
    meal.push_str("Irish potato");
    println!("this is the working meal: {meal}")
    // this is for mutable reference
}
