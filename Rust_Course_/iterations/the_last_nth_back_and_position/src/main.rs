fn main() {
    // let performers = ["Rustful five", "Rust in peace", "Rustin Bieber"];

    // let last = performers.into_iter().last().unwrap();
    // println!("Last: {last:?}");

    // let second = performers.into_iter().nth(1).unwrap();
    // println!("Second: {second:?}");

    // let second_to_last = performers.into_iter().nth_back(1).unwrap();
    // println!("Second to last: {second_to_last:?}");

    // let target_index = performers.into_iter().position(|element| {
    //     element == "Rustin Bieber"
    // });
    // println!("Target index: {target_index:?}");

    let performers = ["Rustful five", "Rust in peace", "Rustin Bieber"];

    let last = performers.iter().last().unwrap();
    println!("Last: {last:?}");

    let second = performers.iter().nth(1).unwrap();
    println!("Second: {second:?}");

    let second_to_last = performers.iter().nth_back(1).unwrap();
    println!("Second to last: {second_to_last:?}");

    let target_index = performers.iter().position(|element| {
        *element == "Rustin Bieber"
    });
    println!("Target index: {target_index:?}");
}
