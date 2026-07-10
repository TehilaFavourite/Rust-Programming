fn main() {
    let stocks = ["nvda", "", "appl", "", "msft", "goog"];

    let capitalized_stocks = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect::<Vec<String>>();
    println!("{}", capitalized_stocks.join(", "));

    let capitalized_shocks = stocks.iter().filter_map(|stock| {
        if stock.is_empty() {
            None
        } else {
            Some(stock.to_uppercase())
        }
    }).collect::<Vec<String>>();
    println!("{}", capitalized_shocks.join(", "));
}
