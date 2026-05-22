fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("second: {}", second);
    first
}

fn main() {
    let orlando  = String::from("Orlando");
    let result = {
        let miami = String::from("Miami");

        longest(&orlando, &miami)
        
    };

}
