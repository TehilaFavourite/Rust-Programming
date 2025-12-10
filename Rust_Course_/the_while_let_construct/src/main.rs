fn main() {
    let mut sauces = vec!["Ketchup", "Mustard", "Mayonnaise"];
    while let Some(sauce) = sauces.pop() {
        println!("I like {} on my hot dogs!", sauce);
    }
}
