mod app;
mod converter;

use app::{App, ConversionMethod};
use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut app = App::new();
    let mut input = String::new();

    if let Some(arg) = env::args().nth(1) {
        app.set_input(arg);
    }

    loop {
        //Menu options for the user
        print!("\n1. Iterative method\n2. Recursive method\n3. Lookup table method (fastest method)\n4. Benchmark all methods\n5. Exit\nInput choice (1-5): ");

        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        if env::args().len() == 1 && choice.trim() != "5" {
            print!("\nEnter a decimal number: ");
            io::stdout().flush()?;
            input.clear();
            io::stdin().read_line(&mut input)?;
            app.set_input(input.clone());
        }

        match choice.trim() {
            "1" => app.set_method(ConversionMethod::Iterative),
            "2" => app.set_method(ConversionMethod::Recursive),
            "3" => app.set_method(ConversionMethod::Lookup),
            "4" => {
                print!("{}", app.benchmark());
                continue;
            }
            "5" => break,
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            }
        }

        match app.convert() {
            Ok(_) => println!("\n{}", app.get_result()),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
