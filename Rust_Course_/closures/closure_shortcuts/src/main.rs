fn main() {
    let multiplier = 5;

    let multiply_by = |value: u8| value * multiplier;
    println!("The result is: {}", multiply_by(3 as u8));

    let mirror = |value| value;
    println!("The mirror says: {}", mirror("Hello"));
    // println!("The mirror says: {}", mirror(10));
}
