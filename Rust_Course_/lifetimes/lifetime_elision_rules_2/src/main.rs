struct DentisAppointment {
    doctor: String,
}

impl DentisAppointment {
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!("check_in_time: {}, check_out_time: {}", check_in_time, check_out_time);
        check_in_time
    }
}


fn main() {
    let appointment = DentisAppointment {
        doctor: String::from("Dr. Smith"),
    };

    let result = appointment.book("10:00 AM", "11:00 AM");
    println!("Booked with: {}", result);
}
