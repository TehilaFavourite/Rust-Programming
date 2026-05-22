fn main() {
    let mut animals = vec!["dog", "cat", "frog"];
    println!("length of last element: {:?}", length_of_last_element(&mut animals));
    println!("length of last element: {:?}", length_of_last_element(&mut animals));
    println!("length of last element: {:?}", length_of_last_element(&mut animals));
    println!("length of last element: {:?}", length_of_last_element(&mut animals));
    

}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}
