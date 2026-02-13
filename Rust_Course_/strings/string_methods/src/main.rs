fn main() {
    // trim
    let mut music_genres = "     Rock, Metal, Country, Rap";
    println!("{}", music_genres.trim());

    // casing
    println!("{}", music_genres.to_uppercase());

    // Replace
    println!("{}", music_genres.replace("a", "@"));

    // split
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
}
