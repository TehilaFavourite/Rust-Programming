pub mod diet {
  pub const NUTRITIONIST: &str = "Norah Nutrition";

  pub fn ask_about_program() {
    println!("The nutritionist is {}", NUTRITIONIST);
  }
}

pub mod weightlifting;
pub mod cardio;

use cardio::CardioTool;

use cardio::Exercise as CardioExercise;
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    pub cardio: CardioExercise,
    pub weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new() -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        let cardio = CardioExercise::new(
            "Thursday".to_string(),
            CardioTool::Bike,
            30,
        );

        let weightlifting = WeightliftingExercise::new(
            "Bench Press".to_string(),
            8,
        );

        GymWorkout {
            cardio,
            weightlifting,
        }
    }
}