#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]

enum Meat {
    Chicken,
    Steak,
}
#[derive(Debug)]

enum RestaurantItem {
    Dish{meat: Meat, beans: Beans},
    Burrito(Meat),
    Bowl(Meat),
    VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito(Meat:: Steak);
    let dinner = RestaurantItem::Bowl(Meat::Chicken);
    let normal = RestaurantItem::VeganPlate;
    println!("launch was {:?}, and dinner was {:?}, while yesterday, we ordered {:?}", lunch, dinner, normal);
    
    let fast_food = RestaurantItem::Dish{meat: Meat:: Chicken, beans:Beans::Black};
    println!("we ate {:?} yesterday", fast_food);
}
