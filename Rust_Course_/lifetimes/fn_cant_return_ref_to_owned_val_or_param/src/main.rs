fn create() -> &i32 {
    let age = 100; 
    &age
}

fn create_slice(items: Vec<i32>) -> &[i32] {
    &items
}
fn main() {
    let age = create(); // This will cause a compile error because create() returns a reference to a value that has been dropped
    println!("Age: {}", age);
}
