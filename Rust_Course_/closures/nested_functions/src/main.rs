fn main() {

    let multiplier = 5;
    fn multiply_by(value: i32) -> i32 {
        value * multiplier // This will cause an error because `multiplier` is not in scope here.
    }
    println!("{}", multiply_by(5));
}


