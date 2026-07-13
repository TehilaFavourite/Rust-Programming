fn main() {
    let first_names = ["Alice", "Bob", "Charlie"];
    let last_names = ["Smith", "Johnson", "Williams"];

    for (first_name, last_name) in first_names.iter().zip(last_names.iter()) {
        println!("{} {}", first_name, last_name);
    }


    let complete_names = first_names
        .iter()
        .zip(last_names.iter())
        .map(|(first_name, last_name)| format!("{} {}", first_name, last_name))
        .collect::<Vec<_>>();
    println!("{:?}", complete_names);
}
