use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Sushi"), 30.00);

    println!("{menu:?}");

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();

    country_capitals.insert("Nigeria", "Abuja");
    country_capitals.insert("Ghana", "Accra");
    country_capitals.insert("France", "Paris");

    println!("{country_capitals:#?}")
}
