use crate::converter;
use num_bigint::BigUint;
use std::{io::Write, ops::Div};

pub enum ConversionMethod {
    Iterative,
    Recursive,
    Lookup,
}

/**
impl block with the following methods:
- [`ConversionMethod::convert`] function to convert the decimal number to binary based on the selected method. It uses a match expression to call the corresponding function from the [`converter`] module
- [`ConversionMethod::get_name`] function to get the name of the method
*/
impl ConversionMethod {
    fn convert(&self, dec: &BigUint) -> String {
        match self {
            ConversionMethod::Iterative => {
                format!("Iterative: {}", converter::decimal_to_binary_iterative(dec))
            }
            ConversionMethod::Recursive => {
                format!("Recursive: {}", converter::decimal_to_binary_recursive(dec))
            }
            ConversionMethod::Lookup => {
                format!("Lookup: {}", converter::decimal_to_binary_lookup(dec))
            }
        }
    }

    fn get_name(&self) -> &str {
        match self {
            ConversionMethod::Iterative => "Iterative",
            ConversionMethod::Recursive => "Recursive",
            ConversionMethod::Lookup => "Lookup",
        }
    }
}

pub struct App {
    input: String,
    method: ConversionMethod,
    result: String,
}

/**
impl block with the following methods:
- [`App::new`] function to create a new instance of App
- [`App::set_method`] function to set the method field
- [`App::convert`] function to convert the input to binary based on the method field
- [`App::benchmark`] function to benchmark the three methods for a given number of iterations
- [`App::get_result`] function to get the result field
*/
impl App {
    /// App struct with the input decimal number, conversion method, and result binary number
    pub fn new() -> App {
        App {
            input: String::new(),
            method: ConversionMethod::Iterative,
            result: String::new(),
        }
    }

    pub fn set_method(&mut self, method: ConversionMethod) {
        self.method = method;
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
    }

    /// Converts the input decimal number to binary based on the selected method
    pub fn convert(&mut self) -> Result<(), String> {
        let dec = self
            .input
            .trim()
            .parse::<BigUint>()
            .map_err(|e| e.to_string())?;

        self.result = self.method.convert(&dec);

        Ok(())
    }

    /// Benchmarks the three conversion methods for a given number of iterations and returns the mean elapsed time
    pub fn benchmark(&self) -> String {
        let dec = self
            .input
            .trim()
            .parse::<BigUint>()
            .expect("Invalid number");

        print!("\nHow many iterations? ");
        let mut input = String::new();
        std::io::stdout().flush().expect("Failed to flush stdout");
        let _ = std::io::stdin().read_line(&mut input);
        let n_it = input.trim().parse::<u32>().expect("Invalid number");

        let methods = vec![
            ConversionMethod::Iterative,
            ConversionMethod::Recursive,
            ConversionMethod::Lookup,
        ];

        let mut results = String::new();
        for method in methods {
            let now = std::time::Instant::now();
            for _ in 0..n_it {
                let _ = method.convert(&dec);
            }
            let elapsed = now.elapsed();
            results.push_str(&format!(
                "\t{} method: {:.2?}\n",
                method.get_name(),
                elapsed.div(n_it)
            ));
        }

        format!("\nMean elapsed time:\n\n{}", results)
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }
}
