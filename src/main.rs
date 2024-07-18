mod app;
mod converter;

use app::{App, ConversionMethod};
use std::env;
use std::io::{self, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut app = App::new();

    let mut input = String::new();

    app.set_input(match env::args().nth(1) {
        Some(arg) => arg,
        None => String::new(),
    });

    loop {
        //Menu options for the user
        println!("\n1. Iterative method");
        println!("2. Recursive method");
        println!("3. Lookup table method (fastest method)");
        println!("4. Exit");
        print!("Choose a method (1-4): ");

        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => app.set_method(ConversionMethod::Iterative),
            "2" => app.set_method(ConversionMethod::Recursive),
            "3" => app.set_method(ConversionMethod::Lookup),
            "4" => break,
            _ => {
                println!("Invalid choice, please try again.");
                continue;
            }
        }

        if env::args().len() == 1 {
            print!("Enter a decimal number: ");
            io::stdout().flush()?;
            input.clear();
            io::stdin().read_line(&mut input)?;
            app.set_input(input.clone());
        }

        //Benchmarking the time taken to convert the decimal number
        let now = Instant::now();
        let result = app.convert();
        let elapsed = now.elapsed();

        if let Err(result) = result {
            println!("Error: {}", result);
        } else {
            println!("{} \n\nElapsed: {:.2?}", app.get_result(), elapsed);
        }
    }

    Ok(())
}
