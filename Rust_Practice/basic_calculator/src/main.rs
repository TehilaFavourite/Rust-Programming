use std::io;

fn main() {
    println!("Hello, this is your basic calculator");

    let first_num = get_first_number();
    let operation = get_operation();
    let second_num = get_second_number();

    match calculate(first_num, second_num, &operation) {
        Ok(result) => {
            println!("{} {} {} = {}", first_num, operation, second_num, result);
            println!("You have successfully used the basic calculator, Thank you!!!");
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn get_first_number() -> f64 {
    loop {
        println!("Enter the first number: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            match input.trim().parse::<f64>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("Invalid float");
                }
        }
    }
}

fn get_operation() -> String {
    loop {
        println!("Enter operation (+, -, *, /): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let operation = input.trim();

        match operation {
            "+" | "-" | "*" | "/" => return operation.to_string(),
            _ => {
                println!("Invalid operation! Please enter +, -, *, or /");
            }
        }
    }
}

fn get_second_number() -> f64 {
    loop {
        println!("Enter the second number: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid float");
            }
        }
    }
}

// fn get_third() -> <f64> {
//     loop {
//         println!("enter the third number");
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         match input.trim().parse::<f64>() {
//             Ok(num) => return num,
//             Err(_) => {
//                 println!("Invalid float");
//             }
//         }
//     }
// }

fn calculate(num1: f64, num2: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}

// println!("Hello, this is your basic calculator");
//     println!("please put in your arithmetic.");

//     let mut input = String::new();
    // let my_string = "3.14";
    // let my_float: f64 = my_string.parse().expect("Failed to parse");
    // println!("this is my float {}", my_float);

    // let my_string = "3.14";
    // let my_float = match my_string.parse::<f64>(){
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Invalid float");
    //         return;
    //     }
    // };
    // println!("this is my float {}", my_float);




    // have a calculator where they enter a value through a url. ------> future project
    // addd the square root, power,

    // Rust
    
    enum TxStatus {
    Pending,
    Success,
    Failed(String),
    }

    struct Transaction {
        id: u32,
        amount: u64,
        status: TxStatus,
    }

    impl Transaction {
        fn fail(&mut self, reason: String) {
            self.status = TxStatus::Failed(reason);
        }
    }


