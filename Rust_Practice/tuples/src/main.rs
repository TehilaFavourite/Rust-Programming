fn main() {
    // creating and destructuring tuples
    // tuples are fixed-size collections of heterogeneous data
    // tuples can contain different types of data
    // tuples can be destructured into variables
    // tuples can be indexed using dot notation
    // tuples can be used to return multiple values from a function
    // tuples can be used to group related data together
    // tuples can be used to create a record-like structure
    // tuples can be used to create a tuple struct
    // tuples can be used to create a tuple enum
    // tuples can be used to create a tuple variant
    // tuples can be used to create a tuple type
    let tuple1: (i32, i32, i32) = (1, 2, 3);
    let tuple2 = (4, 5, 6);
    let tuple3 = (7, 8, 9);
    println!("{:?}, {:?}, {:?}", tuple1, tuple2, tuple3);
    let one = tuple1.0;
    let five = tuple2.1;
    let nine = tuple3.2;
    println!("one: {}, five: {}, nine: {}", one, five, nine);
    let tuple4 = (tuple1.0, tuple2.1, tuple3.2);
    println!("tuple4: {:?}", tuple4);
}
