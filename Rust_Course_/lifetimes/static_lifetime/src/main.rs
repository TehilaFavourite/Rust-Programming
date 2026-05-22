const COUNT: i32 = 100;

fn say_hello() -> &'static str {
    "Hello"
}

fn value() -> &'static i32 {
    &COUNT
}

fn main() {
    let greeting = say_hello();
    println!("{}", greeting);
    println!("Value: {}", value());
}
