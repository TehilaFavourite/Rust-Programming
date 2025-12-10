fn main() {
    let ok: Result<i8, &str> = Result::Ok(5);
    let err: Result<i32, &str> = Result::Err("Something went wrong");
}
