use std::process::exit;

fn check_os() {
    match std::env::consts::OS {
        "widnows" => {
            println!("pussy ass motherfucker get linux then you can use slop");
            exit(69);
        }
        "macos" => {
            println!("you labubu matcha faggot, invest some of that 2000$ macbook into a 500$ laptop with linux, pussy");
            exit(69);
        }
        _ => (),
    }
}

fn main() {
    // Check os only non-windows and non-macos systems are allowed.
    check_os();
    let stdout = std::io::stdout();
}
