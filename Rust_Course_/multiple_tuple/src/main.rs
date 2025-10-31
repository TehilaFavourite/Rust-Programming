fn First_tuple<T, U>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

fn second_tuple<T, U>(first: T, second: T) -> (T, T) {
    (first, second)
}

fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    First_tuple("hello", 5);
    second_tuple("hello", 5);

    make_tuple("hello", 5);
}