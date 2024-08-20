## Decimal to Binary Converter in Rust

![GitHub](https://img.shields.io/github/license/LuMarans30/dec2bin-rust)
![GitHub repo size](https://img.shields.io/github/repo-size/LuMarans30/dec2bin-rust)
![GitHub issues](https://img.shields.io/github/issues/LuMarans30/dec2bin-rust)
![GitHub last commit](https://img.shields.io/github/last-commit/LuMarans30/dec2bin-rust)

A simple Rust implementation of a decimal to binary converter, inspired by my previous [C version](https://github.com/LuMarans30/three-version-decimal-to-binary). 
I'm using this project to learn Rust and explore its features.

## Overview

This project provides a basic CLI for converting decimal numbers to their binary representations.

## Usage

To use this tool, run it from the command line:
```bash
cargo run [optional_decimal_number]
```

Alternatively, [precompiled binaries for most platforms](https://github.com/LuMarans30/dec2bin-rust/releases/latest) are available.

## Conversion Methods

1. **Iterative**: Uses a loop to perform the conversion. It divides the decimal number by 2 iteratively and gets the remainder.
2. **Recursive**: Same algorithm as iterative method, but recursively.
3. **Lookup Table**: Uses a pre-computed lookup table for faster conversion (generally the fastest method).

## Features

- Supports conversion of large numbers using the `num_bigint` crate
- Measures and displays the time taken for each conversion

If you provide a decimal number as an argument, the program will use that for conversions. Otherwise, it will prompt you to enter a number.

## Contributing

This project was created as a learning exercise. While it's primarily for personal use, suggestions and improvements are welcome. Feel free to open an issue or submit a pull request if you have ideas to enhance its functionality.

## License

This project is open-source and available under the [MIT License](LICENSE).
