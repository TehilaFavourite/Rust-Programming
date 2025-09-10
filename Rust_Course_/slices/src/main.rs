fn main() {
    let action_hero = String::from("The incredible Hulk"); // this is a heap allocated string
    let string_ref = &action_hero; //reference
    println!("The string reference is: {string_ref}");
    let first_words = &action_hero[0..14];
    println!("The first words are: {first_words}");

    /* String slices and string literals */
    // let action_hero = "The incredible Hulk"; //string slice
    // let first_words = &action_hero[0..14]; // string slice
    let first_words = {
        let action_hero = "The incredible Hullk";
        &action_hero[0..14];
        // the above would not create a dangling reference
    };

    /* string slice length */
    let food = "Pizza"; // this is a collection of 5 bytes
    println!("The length of the food is: {}", food.len());

    let pizza_slice = &food[0..2];
    println!("The pizza slice is: {pizza_slice}");

    /* syntactic shortcut */

    let repo = String::from("my project work");
    let first_push = &repo[0..3];
    println!("The first push is: {first_push}");
    let second_push = &repo[..11];
    println!("The second push is: {second_push}");
    let last_push = &repo[11..];
    println!("The last push is: {last_push}");
    let full_push =&repo[..];
    println!("The full push is: {full_push}");

    /* string slices as a function parameter */
    do_hero_stuff("The incredible Hulk");


}

/* string slices as a function parameter */
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} has saved the day!");

}
