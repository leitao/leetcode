use std::fs::*;

fn main() -> std::io::Result<()> {
    let filename = "src/main.rs".to_string();

    let lines : Vec<String> = read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect(); // gather them together into a vector


    let mut inside = false;
    let mut m = 0;

    for line in lines {
        if line.contains("{") {
            m += 1;
        }
        if line.contains("}") {
            m -= 1;
        }
        if inside {
            println!("{}", line);
            // We still wnat to process if char = }
        }
        if m == 0 {
            inside = false;
        }

        // Starts

        if line.starts_with("impl Solution") && line.contains("{"){
            inside = true;
            println!("{}", line);
        }

        // Exceptions
        if line.starts_with("use") {
            println!("{}", line);
        }
    }
    Ok(())
}
