#[derive(Debug)]

struct TrainSystem<'a> {
    // name: &str, // This will cause a compilation error because it has a reference without a lifetime annotation
    name: &'a str, // This is fine because it has a lifetime annotation, but we need to specify the lifetime parameter for the struct
}

fn main() {
    let name = String::from("Metro");
    let train_system = {
        
        TrainSystem { name: &name }
    };
    println!("{:?}", train_system);
}
