pub trait TicketSeller {
    fn sell_ticket(&mut self);
}

#[derive(Debug, Eq, PartialEq)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

#[derive(Debug)]
pub struct MovieTheater {
    movies: Vec<String>,
    sales: u32,
}

impl MovieTheater {
    pub fn new() -> Self {
        Self {
            movies: vec![],
            sales: 0,
        }
    }

    pub fn sell_ticket(&mut self) {
        self.sales += 15;
    }

    fn add_movie(&mut self, movie: &str) {
        self.movies.push(movie.to_string());
    }
}

impl TicketSeller for MovieTheater {
    fn sell_ticket(&mut self) {
        self.sales += 25;
    }
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("Museum does not have storage space for another painting")
        }

        self.paintings.push(painting.to_string());
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() >= 2
    }
}

impl TicketSeller for Museum {
    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn result_example() -> Result<(), String> {
    //     Ok(())
    // }

    #[test]
    #[ignore]
    fn hashmaps() {
        let mut one = std::collections::HashMap::new();
        one.insert("a", 1);
        one.insert("b", 2);

        let mut two = std::collections::HashMap::new();
        two.insert("a", 1);
        two.insert("b", 2);

        assert_eq!(one, two);
    }

    // #[test]
    // fn museum_can_sell_ticket_to_increase_revenue() {
    //     let mut museum = Museum::new();
    //     museum.sell_ticket();
    //     assert_eq!(museum.revenue, 25, "The revenue from selling 1 ticket did not match");
    // }

    #[test]
    fn museum_can_sell_ticket_to_increase_revenue() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.sell_ticket();

        if museum.revenue == 25 {
            Ok(())
        } else {
            Err(String::from(
                "the revenue from selling one ticket did not match",
            ))
        }
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 50);
    }

    #[test]
    fn museum_can_have_impressive_art_collection() -> Result<(), String> {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("Starry Night");
        museum.buy_painting("Starry Night");

        if museum.has_impressive_collection() {
            Ok(())
        } else {
            Err(String::from(
                "The museum does not have impressive art collection",
            ))
        }
    }

    #[test]
    fn museum_did_not_sell_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_ne!(museum.revenue, 0);
    }

    #[test]
    fn new_museums_are_equal() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new museum instances were not found to be equal: {museum1:?}"
        );
    }

    #[test]
    #[should_panic(expected = "storage space")]
    fn museum_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("Starry Night");
        museum.buy_painting("Mona");
        museum.buy_painting("Starry");
    }
}
