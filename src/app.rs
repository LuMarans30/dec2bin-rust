use crate::converter;
use num_bigint::BigUint;

/*
* The methods are defined in an enum called ConversionMethod with three variants
*/
pub enum ConversionMethod {
    Iterative,
    Recursive,
    Lookup,
}

impl ConversionMethod {
    fn convert(&self, dec: &BigUint) -> String {
        match self {
            ConversionMethod::Iterative => format!(
                "Iterative: {}",
                converter::decimal_to_binary_iterative(dec)
                    .into_iter()
                    .map(|d| d.to_string())
                    .collect::<String>()
            ),
            ConversionMethod::Recursive => {
                format!("Recursive: {}", converter::decimal_to_binary_recursive(dec))
            }
            ConversionMethod::Lookup => {
                format!("Lookup: {}", converter::decimal_to_binary_lookup(dec))
            }
        }
    }
}

pub struct App {
    input: String,
    method: ConversionMethod,
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

    pub fn convert(&mut self) -> Result<(), String> {
        let dec = self
            .input
            .trim()
            .parse::<BigUint>()
            .map_err(|e| e.to_string())?;

        self.result = self.method.convert(&dec);

        Ok(())
    }

    pub fn get_result(&self) -> &str {
        &self.result
    }
}
