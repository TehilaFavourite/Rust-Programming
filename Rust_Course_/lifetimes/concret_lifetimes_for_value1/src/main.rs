fn main() {
    let a = 1;

    {
        let b = 2; // the lifetime of b is not the main function but the inner scope
    }
    let c = String::from("Tee");
    drop(c); // drop is a function that takes ownership of the value and drops it

}
