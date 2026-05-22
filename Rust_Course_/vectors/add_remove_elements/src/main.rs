fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    println!("{pizza_diameters:?}");

    pizza_diameters.insert(3, 13);
    println!("{pizza_diameters:?}");

    let pizza_diameters = pizza_diameters.pop();
    println!("{pizza_diameters:?}");

    let mut pizza = vec![8, 10, 12, 14];
    let third = pizza.remove(2);
    println!("{pizza:?}");
}
