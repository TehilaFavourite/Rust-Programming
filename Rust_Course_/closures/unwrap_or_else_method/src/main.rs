fn main() {
    let option = Some("Tehila");
    let name = option.unwrap_or_else(|| "best");
    println!("Name: {}", name);

    let option = None;
    let unknown = false;
    let name = option.unwrap_or_else(|| { if unknown { "unknown" } else { "best" } });
    println!("Name: {}", name);
}
