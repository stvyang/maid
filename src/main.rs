use std::io::{self, Write};

fn main() {
    // Command Line
    let mut first_item = String::new();
    print!("Enter the first item keyword: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut first_item).expect("Nope");
    println!("First item: {}", first_item);

    let mut second_item = String::new();
    print!("Enter the second item keyword: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut second_item).expect("Nope");
    println!("Second item: {}", second_item);
}
