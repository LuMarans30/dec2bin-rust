mod app;
mod converter;

use app::App;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut app = App::new();

    loop {
        println!("1. Iterative method");
        println!("2. Recursive method");
        println!("3. Lookup table method (fastest method)");
        println!("4. Exit");
        print!("Choose a method (1-4): ");

        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" | "2" | "3" => {
                use std::time::Instant;
                let now = Instant::now();
                app.set_method(choice.trim().parse::<usize>().unwrap() - 1);
                let elapsed = now.elapsed();
                if let Err(e) = app.convert() {
                    println!("Error: {}", e);
                } else {
                    println!("{} \n\nElapsed: {:.2?}", app.get_result(), elapsed);
                }
            }
            "4" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}
