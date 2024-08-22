mod app;
mod converter;

use app::{App, ConversionMethod};
use std::env;
use std::io::{self, Write};

/**
 * Main function to run the binary converter application with a menu for selecting the conversion method <br>
 * The function reads the user's choice from the standard input and performs the conversion based on the selected method <br>
 * The decimal number is read from the command-line argument if provided, otherwise, it is read from the standard input
 */
fn main() -> io::Result<()> {
    let mut app = App::new();
    let mut input = String::new();

    if let Some(arg) = env::args().nth(1) {
        app.set_input(arg);
    }

    loop {
        print!("\n1. Iterative method\n2. Recursive method\n3. Lookup table method (fastest method)\n4. Benchmark\n5. Exit\nInput choice (1-5): ");

        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        // Checks if the input arg is empty and the choice is not to exit the program (5)
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
                continue; // Skips the conversion operation
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
