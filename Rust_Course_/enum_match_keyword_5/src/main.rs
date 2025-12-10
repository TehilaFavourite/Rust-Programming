#[derive(Debug)]
enum Milk {
    Lowfat (i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("I bought a milk with 2% fat");
            }
            Milk::Lowfat(percent) => {
                println!("you have got a low fat {percent}% of milk version");
            }
            Milk::Whole => {
                println!("You've got the whole milk");
            }
        }
    }
}

fn main() {
    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink();
    Milk::Whole.drink();
}
