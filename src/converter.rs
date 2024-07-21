use std::collections::VecDeque;

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

/*
* Function to convert a decimal number to binary using an iterative approach (loop)
* The function divides the decimal number by 2 iteratively and gets the remainder, which is added to the front of the result vector
*/
pub fn decimal_to_binary_iterative(dec: &BigUint) -> String {
    if dec.is_zero() {
        return "0".to_string();
    }
    let mut result = VecDeque::new();
    let mut n = dec.clone();
    let two = BigUint::from(2u8);
    while !n.is_zero() {
        let bit = (&n % &two).to_u8().unwrap_or(0);
        result.push_front(bit);
        n /= &two;
    }

    result.into_iter().map(|b| b.to_string()).collect()
}

/*
* Function to convert a decimal number to binary using recursion (call stack)
* The function calls itself recursively to divide the decimal number by 2 and get the remainder
*/
pub fn decimal_to_binary_recursive(dec: &BigUint) -> String {
    fn recursive_helper(dec: &BigUint, result: &mut String) {
        if dec > &BigUint::one() {
            recursive_helper(&(dec / 2u8), result);
        }
        result.push_str(&format!("{}", dec % 2u8));
    }

    let mut result = String::new();
    recursive_helper(dec, &mut result);
    result
}

/*
* Function to convert a decimal number to binary using a lookup table
* The lookup table is an array of strings with the binary representation of each hexadecimal digit
* The function converts the decimal number to hexadecimal and then uses the lookup table to get the binary representation
* Due to the use of a lookup table, this method is the fastest amongst the three for any size of decimal number
*/
const BIN_LOOKUP: [u16; 16] = [
    0b0000, 0b0001, 0b0010, 0b0011, 0b0100, 0b0101, 0b0110, 0b0111, 0b1000, 0b1001, 0b1010, 0b1011,
    0b1100, 0b1101, 0b1110, 0b1111,
];

pub fn decimal_to_binary_lookup(dec: &BigUint) -> String {
    if dec.is_zero() {
        return "0".to_string();
    }

    let mut result = Vec::new();
    let mut n = dec.clone();
    let sixteen = BigUint::from(16u8);
    let zero = BigUint::zero();

    while n > zero {
        let index = (&n % &sixteen).to_u8().unwrap();
        let binary = BIN_LOOKUP[index as usize];

        // Push each bit of the 4-bit binary number
        for i in (0..4).rev() {
            result.push(if (binary & (1 << i)) != 0 { '1' } else { '0' });
        }

        n /= &sixteen;
    }

    // Trim leading zeros and reverse the result
    let trimmed: String = result.into_iter().rev().skip_while(|&c| c == '0').collect();

    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed
    }
}
