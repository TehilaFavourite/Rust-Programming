#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn main() {
    let gold_chest = TreasureChest{
        captain:String::from("freebeard"),
        treasure:"Gold",
    };
    println!("the first treasure is {:#?}", gold_chest);
    
    let silver_chest = TreasureChest{
        captain:String::from("Pilates"),
        treasure:"Silver".to_string(),
    };
    println!("the second treasure is {:#?}", silver_chest);
    
    let special_chest = TreasureChest{
        captain:String::from("Pilates"),
        treasure:["Gold", "Silver", "Platinum"],
    };
    println!("the third treasure is {:#?}", special_chest);
}