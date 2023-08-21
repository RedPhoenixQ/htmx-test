use std::process::Command;

const TAILWIND_COMPILE: &'static str = "tailwindcss -i app.css -o static/out.css";

fn main() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", TAILWIND_COMPILE]).output()
    } else {
        Command::new("sh").args(&["-c", TAILWIND_COMPILE]).output()
    }
    .expect("Could not compile tailwindcss");
}
