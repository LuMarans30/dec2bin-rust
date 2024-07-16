use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

pub fn decimal_to_binary_iterative(dec: &BigUint) -> Vec<u8> {
    if dec.is_zero() {
        return vec![0];
    }
    let mut result = Vec::new();
    let mut n = dec.clone();
    let two = BigUint::from(2u8);
    while !n.is_zero() {
        let bit = (&n % &two).to_u8().unwrap_or(0);
        result.push(bit);
        n /= &two;
    }
    result.reverse();
    result
}

pub fn decimal_to_binary_recursive(dec: &BigUint) -> String {
    if dec > &BigUint::one() {
        format!("{}{}", decimal_to_binary_recursive(&(dec / 2u8)), dec % 2u8)
    } else {
        format!("{}", dec % 2u8)
    }
}

pub fn decimal_to_binary_lookup(dec: &BigUint) -> String {
    let bin: [&str; 16] = [
        "0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000", "1001", "1010",
        "1011", "1100", "1101", "1110", "1111",
    ];

    let mut result = String::new();
    let mut n = dec.clone();
    let sixteen = BigUint::from(16u8);
    let zero = BigUint::zero();

    if n.is_zero() {
        return "0000".to_string();
    }

    while n > zero {
        let index = (&n % &sixteen).to_u8().unwrap();
        result = format!("{}{}", bin[index as usize], result);
        n /= &sixteen;
    }

    result.trim().to_string()
}
