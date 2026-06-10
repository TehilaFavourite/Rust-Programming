use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String> {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("1234"),
        treasure: String::from("gold"),
    };

    let hack = || {
        let mut user_input = String::new();
        println!("Enter the vault password: ");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };

    let extraction = vault.unlock(hack);
    println!("extraction: {:?}", extraction);
}
