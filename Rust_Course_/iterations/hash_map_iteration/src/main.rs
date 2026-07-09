use std::collections::HashMap;

fn main() {
    let mut todos = HashMap::new();
    todos.insert("Learn Rust", true);
    todos.insert("Build a project", false);
    todos.insert("Contribute to open source", false);

    for (todo, completion_status) in &mut todos {
        *completion_status = true;
    }
    println!("{:?}", todos);
}
