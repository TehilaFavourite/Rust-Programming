fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    let options = pizza_toppings.get(0);

    match options {
        Some(topping) => println!("the topping is {topping}"),
        None => println!("No value at index"),
    }

}
