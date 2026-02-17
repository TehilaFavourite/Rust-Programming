use std::collections::HashSet;
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Loli");
    concert_queue.insert("Melissa");

    movie_queue.insert("Loli");
    movie_queue.insert("Phil");

    // uinion wil give you the union or combination of of every entry found in both of the set
    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    // the difference method is going to give you the values that are found in the first set, which is the one that the method is invoked upon but not found in the second set

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    // the symmetric difference gives the values that are in either one od the set but not both

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    // the disjoint method returns true if the sets have no valur in common- if there is nothing that is shared between them
    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    // is_subset is going to return true if the that the method is invoked upon is a subset of th argument set
    let mut attendees = HashSet::new();
    attendees.insert("Loli");
    println!("{:?}", concert_queue.is_subset(&movie_queue));
    println!("{:?}", attendees.is_subset(&concert_queue));
    println!("{:?}", concert_queue.is_superset(&concert_queue));
}


