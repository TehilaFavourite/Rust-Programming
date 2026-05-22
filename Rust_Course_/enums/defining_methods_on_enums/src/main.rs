#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot{temperature: u32},
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
        LaundryCycle::Cold => {
            println!("I am doing a cold laundry");
        },
        LaundryCycle::Hot{temperature} => {
            println!("I am running the laundry with this temperature");
        },
        LaundryCycle::Delicate(fabric_type) => {
            println!("I am running the laundry with a delicate cycle for {fabric_type}");
        }
    }
    }
}
    

fn main() {
    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot{temperature: 100};
    hot_cycle.wash_laundry();
    let delicate_cycle = LaundryCycle::Delicate(String::from("silk"));
    delicate_cycle.wash_laundry();
}
