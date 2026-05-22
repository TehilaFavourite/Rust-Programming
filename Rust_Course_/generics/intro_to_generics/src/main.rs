fn identity<T>(value: T) -> T {
    value
}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(5.5));
    println!("{}", identity("Girls"));
    println!("{}", identity("Girls".to_string()));
}