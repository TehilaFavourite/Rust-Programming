use std::collections::HashMap;

fn count_characters(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }
    counts
}

fn main() {
    println!("{:?}", count_characters("hello world hello rust"));
}
