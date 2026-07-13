fn main() {
    let fifty_numbers = 1..=50;

    // for number in fifty_numbers.take(15) {
    //     println!("Number: {number}");
    // }

    // for number in fifty_numbers.rev().take(15) {
    //     println!("Number: {number}");
    // }

    // for number in fifty_numbers.skip(15).take(15) {
    //     println!("Number: {number}");
    // }

    // for number in fifty_numbers.take(15).skip(5) {
    //     println!("Number: {number}");
    // }

    // for number in fifty_numbers.take(15).skip(5).step_by(2) {
    //     println!("Number: {number}");
    // }

    for number in fifty_numbers.clone().take(15).skip(5).step_by(2) {
        println!("Number: {number}");
    }

    println!("Number: {fifty_numbers:?}");


}
