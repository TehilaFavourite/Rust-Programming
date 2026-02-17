use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("oat milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat white", "Almond Milk");

    coffee_pairings.entry("Latte").or_insert("pistacho Milk");
    println!("{coffee_pairings:?}");
    // the above will achieve nothing because latte is present. see below
    coffee_pairings.entry("Cappucino").or_insert("pistacho Milk");
    println!("{coffee_pairings:?}");

}