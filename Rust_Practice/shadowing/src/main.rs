fn main() {
    let a = 10;
    let a = a + 5; // shadowing
       {
    let a = a + 30; // shadowing
        {
            let a = a + 100; // shadowing
            println!("inner a: {}", a);
        }
        println!("outer a: {}", a);
    }
    let a = 50; // new shadowing
    println!("final a: {}", a); // additional print statement for clarity
    println!("outer a after shadowing: {}", a); // added print statement for clarity
}
