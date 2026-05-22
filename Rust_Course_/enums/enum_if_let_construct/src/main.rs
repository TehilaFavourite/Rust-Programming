#[derive(Debug)]
enum Milk {
    Lowfat (i32),
    Whole,
    NonDiary{kind: String},
}


fn main() {
    let my_beverage = Milk::Whole;
    
    if let Milk::Whole = my_beverage {
        println!("you have the wholw milk");
    }
    
    let your_drink = Milk::Lowfat(2);
    if let Milk::Lowfat(percent) = your_drink {
        println!("your drink is {percent}% milk");
    }
    
    let our_drink = Milk::NonDiary{kind: String::from("creamer")};
    if let Milk::NonDiary{kind} = my_beverage {
        println!("our drink is non diary milk");
    } else {
        println!("you have some other kind of milk")
    }
}
