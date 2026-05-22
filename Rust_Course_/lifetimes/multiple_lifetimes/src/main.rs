#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str,
}

fn main() {
    let from = String::from("Orlando");

    let plan = figure_out_ending_point(&from);
    println!("{}", plan);
}

fn figure_out_ending_point(from: &str) -> &str {
    let to = String::from("Miami");
    
    let travel_plan = TravelPlan { from, to: &to };
    travel_plan.from
}
