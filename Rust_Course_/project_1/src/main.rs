const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "Laliga Season";
    println!("My favourite season is {season}");
    let mut points_scored: i32 = 28;
    println!("the points scored this season is {}", points_scored);
    points_scored = 35;
    println!("another points scored is {0}", points_scored);
    let event_time: &str = "06:00";
    println!("the event time is {event_time}");
    let event_time: i32 = 6;
    println!(
        "My favourite season is {season}, the points scored this season is {0}, another points scored is {0}, the event time is {1}. the event time is {1}, and this is a touch down points: {TOUCHDOWN_POINTS} ",
        points_scored, event_time
    );
    #[allow(unused_variables)]
    let favorite_beverage = "water";
}

/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.

Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.

It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.

Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.

Declare a `event_time` variable set to a string of
"06:00".

Use variable shadowing to redeclare `event_time` set
to a integer of 6.

Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice using direct interpolation {value}, sequential
arguments ( {} ), and numeric arguments ( {0} ).

Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.

Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/
