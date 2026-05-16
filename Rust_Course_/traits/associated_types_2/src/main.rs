use std::ops::Add;

fn add_two_numbers<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let sum = add_two_numbers(5, 10);
    println!("Sum: {}", sum);
    let sum_floats = add_two_numbers(3.5, 2.5);
    println!("Sum of floats: {}", sum_floats);
}
