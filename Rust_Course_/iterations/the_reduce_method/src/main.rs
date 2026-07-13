fn main() {
    // let earnings = vec![100, 200, 300, 400, 500];

    let earnings: [i32; 0] = [];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("Sum: {sum:?}");

    let address_portions = [
        String::from("123 Elm Street"),
        String::from("Apt. 4B"),
        String::from("Springfield"),
    ];

    println!("{}", address_portions.join(", "));

    let address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });
        println!("Address: {address:?}");
}
