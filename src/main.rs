use std::io;

mod string_parser;
mod tests;

fn main() {
    println!("Enter something, will ya?:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)  // Send the reference to input
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(num) => {
            println!("The parsed integer is: {}", num)
        },
        Err(e) => println!("Failed to parse the number: {}", e),
    }
}

// fn another_function(input: &String) -> String {
//     return format!("Another {}! How quaint...", input.trim());
// }