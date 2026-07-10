fn main() {
    let names = [
        String::from("Bob"),
        String::from("Frank"),
        String::from("Ferris"),
    ];

    let name_lengths = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len()).collect::<Vec<usize>>();

    println!("{name_lengths:?}");
}
