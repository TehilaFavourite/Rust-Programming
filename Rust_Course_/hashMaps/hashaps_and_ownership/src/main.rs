use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("oat milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat white", "Almond Milk");
    println!("{:?}", coffee_pairings);
    println!("{}", coffee_pairings.len());
    println!("{drink} {milk}");
}
