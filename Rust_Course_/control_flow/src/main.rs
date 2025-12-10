fn main() {
    let bought = true;
    the_if_statement(bought);
    let mut season = "summer";
    the_else_if_statement(season);
    let season = "winter";
    the_else_if_statement(season);
    let school = "closed";
    the_else_statment(school);
    assigning_result_of_if_statement_to_variable(17);
    assigning_result_of_if_statement_to_variable(20);
    let evaluation = true;
    the_match_statement(evaluation);
    let evaluating = true;
    the_match_statement2(false);
    let pet = "dog";
    underscore_in_match(pet);
    let figures = 2;
    match_statement_with_multiple_values_and_conditionals(figures);
    let morefigures = 15;
    match_statement_with_multiple_values_and_conditionals2(morefigures);
    let time = 20;
    loop_and_break(time);
    let countdown = 20;
    the_continue(countdown);
    let countdown2 = 20;
    while_loop(countdown2);
    // let record = 30;
    recursion(5)

}

fn the_if_statement(bought: bool) {
    if bought {
        println!("the output will be true");
    }
    if !bought {
        println!("the output will be false");
    }
}

fn the_else_if_statement(season: &str) {
    if season == "summer" {
        println!("the summer is nice");
    } else if season == "winter" {
        println!("the winter is too cold");
    }
}

fn the_else_statment(school: &str) {
    if school == "open" {
        println!("school is in session");
    } else {
        println!("we are enjoying our holidays");
    }
}

fn assigning_result_of_if_statement_to_variable(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("the number is {result}")
}

fn the_match_statement(evaluation: bool) {
    match evaluation {
        true => {
            println!("the value is true");
        }
        false => {
            println!("the value is false");
        }
    }
}

fn the_match_statement2(evaluating: bool) {
    let value = match evaluating {
        true => 20,
        false => 40,
    };
    println!("{value}");
}

fn underscore_in_match(pet: &str) {
    match pet {
        "dog" => println!("it's a dog"),
        "cat" => println!("it's a cat"),
        _ => println!("it'a any pet at all"),
    }
}

fn match_statement_with_multiple_values_and_conditionals(figures: i32) {
    match figures {
        2 | 4 | 6 | 8 => println!("{figures} is even"),
        1 | 3 | 5 | 7 => println!("{figures} is odd"),
        _ => println!("{figures} is unknown"),
    }
}

fn match_statement_with_multiple_values_and_conditionals2(morefigures: i32) {
    match morefigures {
        x if x % 2 == 0 => println!("{x} is even"),
        y if y % 2 != 0 => println!("{y} is odd"),
        _ => println!("{morefigures} is unknown"),
    }
}

fn loop_and_break(mut time: i32) {
    loop {
        if time == 0 {
            println!("blastOff!!!");
            break;
        }
        println!("{time} time is even number, skipping 3 seconds");
        time -= 1;
    }
}

fn the_continue(mut countdown: i32) {
    loop {
        if countdown <= 0 {
            println!("blastOff!!!");
            break;
        }
        if countdown % 2 == 0 {
            println!("{countdown} countdown is even number, skipping 3 seconds");
            countdown -=3;
            continue;
        }
        println!("{countdown} countdown is odd, decrementing by 1");
        countdown -= 1;
    }
}

fn while_loop(mut countdown2: i32) {
    while countdown2 > 0 {
        if countdown2 % 2 == 0 {
            println!("{countdown2} countdown is even number and will skip by 3 seconds");
            countdown2 -= 3;
            continue;
        }
        println!("{countdown2} countdown to blast off");
        countdown2 -=1;
    }
    println!("blast off!")
}

fn recursion(record: i32) {
    if record == 0 {
        println!("blast off!!!")
    } else {
        println!("{record} to blast off");
        recursion(record - 1)
    }
}
