use std::collections::HashMap;

fn main() {
    let mut sauces_to_meals = HashMap::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ]);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);
    let remove_sauce = sauces_to_meals.remove("Mayonnaise");

    if let Some(meals) = remove_sauce {
        println!("Removd Mayonnaise meals: {:?}", meals);
    } else {
        println!("Mayonnaise was not found");
    }

    if let Some(mustard_meals) = sauces_to_meals.get("Mustard") {
        println!("Meals for Mustard: {:?}", mustard_meals);
    }

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);
}
