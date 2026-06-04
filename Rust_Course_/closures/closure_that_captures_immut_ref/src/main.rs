fn main() {
    let multiplier = 5;

    let multiply_by = |value: u8| value * multiplier;
    println!("The result is: {}", multiply_by(3 as u8));

    let numbers = vec![4, 8, 15, 16, 23, 42];
    println!("The numbers are: {:?}", numbers);
    let print_numbers = || println!("The numbers are: {:?}", numbers);
    print_numbers();
}
