fn main() {
    let current_meal = String::new();
    add_flour(current_meal);
    let current_meal = String::new();
    show_my_meal(current_meal);
    let current_meal = String::new();
    show_my_meal2(&current_meal);
    let mut current_meal = String::new();
    add_flour2(&mut current_meal);

    
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
