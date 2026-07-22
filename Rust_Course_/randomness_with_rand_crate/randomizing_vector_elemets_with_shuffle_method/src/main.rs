use rand::rng;
use rand::seq::SliceRandom;
fn main() {
    let mut my_rng = rng();
    let mut candies = vec!["Sour patch kids", "Kit Kat", "Twix", "Snickers", "Starburst"];
    candies.shuffle(&mut my_rng);

    println!("{:?}", candies);
}
