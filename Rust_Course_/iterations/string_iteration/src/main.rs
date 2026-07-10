fn main() {
    let seafood = "oyster 🦪";

    // for byte in seafood.bytes() {
    //     println!("{byte}/");
    // }

    // for character in seafood.chars() {
    //     println!("{character}/");
    // }
    // println!("{seafood}");

    println!("{}", seafood.bytes().len());
    println!("{}", seafood.chars().count());
}