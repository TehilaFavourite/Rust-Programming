fn main() {
    let mut flavors = vec![String::from("Chocolate"), String::from("Vanilla"), String::from("Strawberry")];


    for flavor in &mut flavors {
        flavor.push_str(" ice cream");
    }

    let mut school_grades = vec![85, 92, 78, 96, 88];

    for grade in &mut school_grades {
        *grade += 5;
    }

    println!("{school_grades:?}");

}
