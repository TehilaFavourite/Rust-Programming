fn main() {
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);
    // using the turbofish
    let d = Option::<i16>::Some(5);
    let e: Option<&str> = Option::None;
}
