use intro_to_testing::attractions::{MovieTheater, Museum};
use intro_to_testing::management::VenueManagement;
use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("Mona");
    museum.buy_painting("Lisa");
    museum
}

#[fixture]
fn museum_management(museum_with_three_paintings: Museum) -> VenueManagement<Museum> {
    VenueManagement::new(museum_with_three_paintings)
}

#[fixture]
fn movie_theater_with_one_movie() -> MovieTheater {
    let mut movie_theater: MovieTheater = MovieTheater::new();
    movie_theater.add_movie("legally blonde");
    movie_theater
}

#[fixture]
fn movie_theater_management(
    movie_theater_with_one_movie: MovieTheater,
) -> VenueManagement<MovieTheater> {
    VenueManagement::new(movie_theater_with_one_movie)
}

#[rstest]
fn venue_management_interacts_with_museum_venue(museum_with_three_paintings: Museum) {
    let mut venue_mgt = VenueManagement::new(museum_with_three_paintings);
    venue_mgt.make_money();

    assert_eq!(venue_mgt.venue.paintings.len(), 3);
    assert_eq!(venue_mgt.venue.revenue, 25);
}

#[rstest]
fn venue_management_interacts_with_movie_theater_venue(
    mut movie_theater_management: VenueManagement<MovieTheater>,
) {
    movie_theater_management.make_money();
    assert_eq!(movie_theater_management.venue.sales, 25);
}
