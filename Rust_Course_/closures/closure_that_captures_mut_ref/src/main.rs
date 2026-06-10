fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(108);
    add_number();
    println!("The numbers are: {:?}", numbers);
}
