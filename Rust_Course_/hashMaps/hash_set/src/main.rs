use std::collections::HashSet;
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);
    println!("{:?}", concert_queue.len());

    // prevents duplicate
    concert_queue.insert("Megan");
    println!("{:?}", concert_queue);

    // this remove returns true because it exists
    println!("{:?}", concert_queue.remove("Megan"));
    // this remove returns false because it doesn't exists
    println!("{:?}", concert_queue.remove("Frank"));

    println!("{}", concert_queue.contains("Molly"));
    println!("{}", concert_queue.contains("Frank"));

    println!("{:?}", concert_queue.get("Molly"));
    println!("{:?}", concert_queue.get("Frank"));
}
