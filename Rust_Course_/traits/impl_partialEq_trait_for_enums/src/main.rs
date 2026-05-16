// #[derive(PartialEq)]
enum Musician {
    SingerSongwriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongwriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SingerSongwriter(name1), SingerSongwriter(name2)) => name1 == name2,
            (Band(num1), Band(num2)) => num1 == num2,
            _ => false,
        }
    }
}

fn main() {
    let musician1 = SingerSongwriter("Taylor Swift".to_string());
    let musician2 = SingerSongwriter("Taylor Swift".to_string());
    let musician3 = Band(4);
    println!("Is musician1 equal to musician2? {}", musician1 == musician2);
    println!("Is musician1 equal to musician3? {}", musician1 == musician3);
    println!("Is musician2 equal to musician3? {}", musician2 == musician3);
}
