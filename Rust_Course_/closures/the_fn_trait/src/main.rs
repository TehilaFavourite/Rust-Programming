fn execute_thrice<F>(mut procedure: F) where F: FnMut() { 
    procedure();
    procedure();
    procedure();
}

fn main() {
    let mut closures = vec!["best", "greatest", "most amazing"];
    let closure = || {
        closures.push("best");
    };
    execute_thrice(closure);
}
