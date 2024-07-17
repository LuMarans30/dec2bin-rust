use crate::converter;
use num_bigint::BigUint;
use std::io::{self, Write};

pub struct App {
    input: String,
    method: usize,
    result: String,
}

/*
* impl block with the following methods:
* new() function to create a new instance of App
* set_method() function to set the method field
* convert() function to convert the input to binary based on the method field
* get_result() function to get the result field
*/

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            method: 0,
            result: String::new(),
        }
    }

    pub fn set_method(&mut self, method: usize) {
        self.method = method;
    }

    pub fn convert(&mut self) -> Result<(), String> {
        print!("\nEnter a decimal number: ");
        io::stdout().flush().unwrap();
        self.input.clear();
        io::stdin()
            .read_line(&mut self.input)
            .map_err(|e| e.to_string())?;

        let dec = self
            .input
            .trim()
            .parse::<BigUint>()
            .map_err(|e| e.to_string())?;

        self.result = match self.method {
            0 => format!(
                "Iterative: {}",
                converter::decimal_to_binary_iterative(&dec)
                    .iter()
                    .map(|&d| d.to_string())
                    .collect::<String>()
            ),
            1 => format!(
                "Recursive: {}",
                converter::decimal_to_binary_recursive(&dec)
            ),
            2 => format!("Lookup: {}", converter::decimal_to_binary_lookup(&dec)),
            _ => unreachable!(),
        };

        Ok(())
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }
}
