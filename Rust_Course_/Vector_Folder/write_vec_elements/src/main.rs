fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olive");
    println!("Available pizza toppings: {:?}", pizza_toppings);

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Green Peppers");
    println!("Updated pizza toppings: {:?}", pizza_toppings);
}
