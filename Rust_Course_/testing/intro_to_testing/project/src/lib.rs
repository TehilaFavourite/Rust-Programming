/*
We're running a salad restaurant! You discover some starter code
from a previous developer working at the company. The code includes:
- A Vegetable enum
- A Protein enum
- A Dressing enum
*/

trait Caloric {
    fn calories(&self) -> u32;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Vegetable {
    Tomato,
    Cucumber,
    SweetPotato,
}

impl Caloric for Vegetable {
    fn calories(&self) -> u32 {
        match self {
            Self::Tomato => 20,
            Self::Cucumber => 15,
            Self::SweetPotato => 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Protein {
    CrispyChicken,
    FriedChicken,
    Steak,
    Tofu,
}

impl Caloric for Protein {
    fn calories(&self) -> u32 {
        match self {
            Self::CrispyChicken => 400,
            Self::FriedChicken => 500,
            Self::Steak => 300,
            Self::Tofu => 200,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dressing {
    Ranch,
    Vinaigrette,
    Italian,
}

impl Caloric for Dressing {
    fn calories(&self) -> u32 {
        match self {
            Self::Ranch => 150,
            Self::Vinaigrette => 120,
            Self::Italian => 130,
        }
    }
}

// ---------------------------------------------------------
// "Our next goal is to build a Salad struct. Each Salad will
// have a 'protein', 'vegetables', and a 'dressing' field.
// A Salad can store 1 protein, any number of vegetables, and
// 1 dressing. Use a vector to store the vegetables. Derive
// the Debug trait."
// ---------------------------------------------------------
#[derive(Debug)]
struct Salad {
    protein: Protein,
    vegetables: Vec<Vegetable>,
    dressing: Dressing,
}

impl Salad {
    // ---------------------------------------------------------
    // "First, define a 'new' constructor function that accepts
    // a 'protein', a 'vegetables' vector, and a 'dressing' and
    // returns an instance of the Salad."
    // ---------------------------------------------------------
    fn new(protein: Protein, vegetables: Vec<Vegetable>, dressing: Dressing) -> Self {
        Self {
            protein,
            vegetables,
            dressing,
        }
    }

    // ---------------------------------------------------------
    // "Next, define an 'is_valid' method that returns a Boolean.
    // Return true if a salad has more than 0 vegetables."
    // ---------------------------------------------------------
    fn is_valid(&self) -> bool {
        !self.vegetables.is_empty()
    }

    // ---------------------------------------------------------
    // "Next, define a 'calories' method that calculates the
    // total calories in the salad. The Vegetable, Protein, and
    // Dressing enums all support a 'calories' method that return
    // the calories of the item. Remember that 'vegetables' is a
    // vector of multiple Vegetable values -- you'll have to
    // include all of them in your calculation."
    // ---------------------------------------------------------
    fn calories(&self) -> u32 {
        let vegetable_calories: u32 = self.vegetables.iter().map(|v| v.calories()).sum();
        self.protein.calories() + self.dressing.calories() + vegetable_calories
    }

    // ---------------------------------------------------------
    // "Finally, define a 'has_duplicate_vegetables' method. It
    // should determine if the salad includes any vegetable more
    // than once. Return a Boolean."
    // ---------------------------------------------------------
    fn has_duplicate_vegetables(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for vegetable in &self.vegetables {
            // HashSet::insert returns false if the value was
            // ALREADY in the set. So the first time we see a
            // vegetable, insert() returns true and we move on.
            // The second time we see the same one, insert()
            // returns false, which means we found a duplicate.
            if !seen.insert(vegetable) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let salad = Salad::new(
            Protein::Steak,
            vec![Vegetable::Tomato, Vegetable::Cucumber],
            Dressing::Ranch,
        );

        assert_eq!(salad.protein, Protein::Steak);
        assert_eq!(
            salad.vegetables,
            vec![Vegetable::Tomato, Vegetable::Cucumber]
        );
        assert_eq!(salad.dressing, Dressing::Ranch);
    }

    #[test]
    fn test_is_valid() {
        let valid_salad = Salad::new(Protein::Tofu, vec![Vegetable::Tomato], Dressing::Italian);
        assert!(valid_salad.is_valid());

        let invalid_salad = Salad::new(Protein::Tofu, vec![], Dressing::Italian);
        assert!(!invalid_salad.is_valid());
    }

    #[test]
    fn test_calories() {
        let salad = Salad::new(
            Protein::Steak,                               // 300
            vec![Vegetable::Tomato, Vegetable::Cucumber], // 20 + 15 = 35
            Dressing::Ranch,                              // 150
        );

        // 300 + 35 + 150 = 485
        assert_eq!(salad.calories(), 485);
    }

    #[test]
    fn test_has_duplicate_vegetables() {
        let salad_with_dupes = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Tomato, Vegetable::Tomato],
            Dressing::Italian,
        );
        assert!(salad_with_dupes.has_duplicate_vegetables());

        let salad_without_dupes = Salad::new(
            Protein::Tofu,
            vec![Vegetable::Tomato, Vegetable::Cucumber],
            Dressing::Italian,
        );
        assert!(!salad_without_dupes.has_duplicate_vegetables());
    }
}
