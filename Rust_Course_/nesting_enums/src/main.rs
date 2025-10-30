#[derive(Debug)]

enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]

enum RestaurantItem {
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito(Meat:: Steak);
    let dinner = RestaurantItem::Bowl(Meat::Chicken);
    let normal = RestaurantItem::VeganPlate;
    println!("launch was {:?}, and dinner was {:?}, while yesterday, we ordered {:?}", lunch, dinner, normal);
}
