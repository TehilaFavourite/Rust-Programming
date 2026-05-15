// use super::lodging::{Accomodation, Description};
use crate::lodging::{Accomodation, Description};

pub fn book_for_one_night<T: Accomodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accomodation + Description,
    U: Accomodation,
{
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}