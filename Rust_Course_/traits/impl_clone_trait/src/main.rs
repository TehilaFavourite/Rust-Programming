use std::clone::Clone;

#[derive(Clone)]
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

// impl Clone for Appointment {
//     fn clone(&self) -> Self {
//         Self {
//             doctor: self.doctor.clone(),
//             start_time: self.start_time.clone(),
//             end_time: self.end_time.clone(),
//         }
//     }
// }

fn main() {
    let morning_appointment = Appointment::new("Dr. Smith", "9:00 AM", "10:00 AM");
    let replacement_appointment = morning_appointment.clone();
    println!("Morning appointment with {} from {} to {}", morning_appointment.doctor, morning_appointment.start_time, morning_appointment.end_time);
    println!("Replacement appointment with {} from {} to {}", replacement_appointment.doctor, replacement_appointment.start_time, replacement_appointment.end_time);
}
