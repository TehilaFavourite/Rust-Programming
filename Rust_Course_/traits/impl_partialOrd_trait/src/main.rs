use std::cmp::Ordering;

struct Job {
    salary: u32,
    commute_time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.salary.partial_cmp(&other.salary)
    }
    //     if self.salary > other.salary {
    //         Some(Ordering::Greater)
    //     } else if self.salary < other.salary {
    //         Some(Ordering::Less)
    //     } else if self.salary == other.salary {
    //         Some(Ordering::Equal)
    //     } else {
    //         None
    //     }
    // }
}
fn main() {
    let long_commute_job = Job {
        salary: 150000,
        commute_time: 60,
    };
    let short_commute_job = Job {
        salary: 100000,
        commute_time: 30,
    };
    println!("Is long_commute_job greater than short_commute_job? {}", long_commute_job > short_commute_job);
    println!("Is long_commute_job less than short_commute_job? {}", long_commute_job < short_commute_job);
    println!("Is long_commute_job equal to short_commute_job? {}", long_commute_job == short_commute_job);
    println!("Is long_commute_job not equal to short_commute_job? {}", long_commute_job != short_commute_job);
    println!("Partial comparison: {:?}", long_commute_job.partial_cmp(&short_commute_job));

}
