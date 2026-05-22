fn main() {
    let dog = String::from("Watson");

    {
        let my_pet = &dog; 
        println!("My pet's name is: {}", my_pet);
    }
    println!("The dog's name is: {}", dog); 

    {
        let my_pet = &dog; 
        println!("My pet's name is: {}", my_pet);
    }
    
}
