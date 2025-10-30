#[derive(Debug)]
enum Milk {
    Lowfat (i32),
    Whole,
    NonDiary{kind: String},
}


fn main() {
    let my_beverage = Milk::Whole;
    
    let Milk::Lowfat(percent) = my_beverage else {
        println!("you do not have the low fat milk.");
        return;
    };
    
    println!("{percent}% milk is available here")
    
    
}
