
fn main() {
    // experiment with integer overflow scenarios
    let large_int = std::i32::MAX;
    let result = large_int + 1;
    println!("Integer overflow: {}", result);

    // experiment with floating point overflow scenarios
    let large_float = std::f64::MAX;
    let result = large_float + 1.0;
    println!("Floating point overflow: {}", result);
    // result: OverflowError
    // Note: This error is expected due to the limitations of floating point arithmetic
    //       and can't be avoided in this simple program.

}
