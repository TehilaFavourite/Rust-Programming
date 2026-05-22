#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("quite an old operating system");
            39
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34
    }
}

fn main() {
    let my_computer = OperatingSystem::Windows;
    let age = years_since_release(my_computer);
    println!("my computer's operating system is {age} years old");
    
    let mum_computer = OperatingSystem::Windows;
    let the_age = years_since_release(mum_computer);
    println!("my mum's computer system is {the_age} years old");
    
}
