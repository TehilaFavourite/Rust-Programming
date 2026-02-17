use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("oat milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat white", "Almond Milk");
    
    let value = coffee_pairings["Flat white"];
    // the above will panic if the key is not found. See the method below
    let value = coffee_pairings.get("Cappucino").copied().unwrap_or("unknown milk");
    println!("{}", value);
}
