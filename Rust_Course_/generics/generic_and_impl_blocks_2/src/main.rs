#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest = TreasureChest{
        captain:String::from("freebeard"),
        treasure:"Gold",
    };
    println!("the first treasure is {:#?}", gold_chest.capital_captain());
    
    let mut silver_chest = TreasureChest{
        captain:String::from("Pilates"),
        treasure:("Silver"),
    };
    println!("the second treasure is {:#?}", silver_chest);
    
    let special_chest = TreasureChest{
        captain:String::from("Pilates"),
        treasure:["Gold", "Silver", "Platinum"],
    };
    println!("the third treasure is {:#?}", special_chest);
}