#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot{temperature: u32},
    Delicate(String),
}

fn wash_laundry(cycle: LaundryCycle) {
    match cycle {
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

fn main() {
    wash_laundry(LaundryCycle::Cold);
    wash_laundry(LaundryCycle::Hot{temperature: 100});
    wash_laundry(LaundryCycle::Delicate(String::from("silk")));
}
