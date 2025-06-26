const TAX_RATE: f64 = 7.25;
type Meters = i32;

fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    println!("I have {} apples", apples);
    println!("I have {oranges} oranges");
    println!("I have {} apples, {} oranges, and in total {} fruits in my garden", apples, oranges, fruits);
    println!("I have {1} oranges, {0} apples, and in total {2} fruits in my garden. I honestly cannot believe that I have {0} apples", apples, oranges, fruits);

    let mut streak = 30;
    println!("I have {streak} streak in coding in Rust");
    streak = 365;
    println!("now, I have {streak} streak coding in Rust");

    let ton_of_co2 = "0.45";
    println!("{ton_of_co2} is the ton of co2");
    let ton_of_co2 = 0.45;
    println!("{ton_of_co2} is the ton of co2");
    let ton_of_co2 = 450;
    println!("{ton_of_co2} is the ton of co2");

    println!("the tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1000;
    let two_mile_race_length: Meters = 1600;
    println!("a one mile race length is {mile_race_length} long and a two mile race length is {two_mile_race_length} long");

}
