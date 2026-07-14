#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main () {
    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());
    
    points.sort();
    
    println!("{points:?}");
    
    println!("{}", points.is_sorted());
    
    points.reverse();
    println!("{points:?}");
    
    println!("{}", points.is_sorted());
    
    let mut exercises = ["Squat", "Bench", "Deadlift"];
    exercises.sort();
    println!("{exercises:?}");
    
    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Mali Moore"),
        employee_count: 5,
    };
    
    let total = GasStation {
        snack_count: 10,
        manager: String::from("Hoop Lora") ,
        employee_count: 10,
    };
    
    let shell = GasStation {
        snack_count: 20,
        manager: String::from("Ket Pao"), 
        employee_count: 3,
    };
    
    let mut stops = [mobil, total, shell];
    // stops.sort_by_key(|station| {station.snack_count});
    stops.sort_by_key(|station| -(station.employee_count as i32));
    println!("{stops:?}");
}