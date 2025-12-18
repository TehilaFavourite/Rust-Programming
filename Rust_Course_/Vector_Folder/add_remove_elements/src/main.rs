fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    println!("{pizza_diameters:?}");

    pizza_diameters.insert(3, 13);
    println!("{pizza_diameters:?}");
}
