fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    let value = pizza_diameters[2];
    println!("{value}");

    let reference = &pizza_toppings[2];
    println!("{reference}");

    let pizza_slice = &pizza_diameters[1..3];
    println!("{pizza_slice:?}");
}
