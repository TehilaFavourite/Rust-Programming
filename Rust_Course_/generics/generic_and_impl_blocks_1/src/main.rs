#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string()
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

fn main() {
    let gold_chest = TreasureChest{
        captain:String::from("freebeard"),
        treasure:"Gold",
    };
    println!("the first treasure is {:#?}", gold_chest);
    
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