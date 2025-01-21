mod helpers;
use clap::{arg, command, error::ErrorKind};
use helpers::{binary_to_decimal, binary_to_hex, binary_to_octal, decimal_to_2s_complement, decimal_to_unsigned, hex_to_binary, octal_to_binary}; // value_parser, ArgAction, Command

#[derive(Debug)]
enum NumberError {
    InvalidChars(String, Vec<usize>), // Store both the string and error positions
    InvalidFormat(String),
}

impl From<std::num::ParseIntError> for NumberError {
    fn from(_err: std::num::ParseIntError) -> Self {
        // Map the ParseIntError to InvalidFormat
        // You can customize this mapping if needed
        NumberError::InvalidFormat("Failed to parse number".to_string())
    }
}

fn find_invalid_positions(s: &str, valid_chars: impl Fn(char) -> bool) -> Vec<usize> {
    s.char_indices()
        .filter(|&(_, c)| !valid_chars(c))
        .map(|(i, _)| i)
        .collect()
}

fn build_hex_respose(hex: &str) -> Result<String, std::num::ParseIntError> {
    let binary = hex_to_binary(hex)?;
    let mut response = String::new();
    response.push_str(&format!("Hexadecimal Input: {}\n\n", hex));
    response.push_str(&format!("Binary Representation: {}\n", binary));
    response.push_str(&format!(
        "Octal Representation: {}\n",
        &binary_to_octal(&binary)?
    ));
    response.push_str(&format!("Decimal:\n{}", &binary_to_decimal(&binary)?));
    Ok(response)
}

fn build_binary_respose(binary: &str) -> Result<String, std::num::ParseIntError> {
    let mut response = String::new();
    response.push_str(&format!("Binary Input: {}\n\n", binary));
    response.push_str(&format!(
        "Hexadecimal Representation: {}\n",
        &binary_to_hex(&binary)?
    ));
    response.push_str(&format!(
        "Octal Representation: {}\n",
        &binary_to_octal(&binary)?
    ));
    response.push_str(&format!("Decimal:\n{}", &binary_to_decimal(&binary)?));
    Ok(response)
}

fn build_octal_respose(octal: &str) -> Result<String, std::num::ParseIntError> {
    let mut response = String::new();
    let binary = octal_to_binary(octal)?;
    response.push_str(&format!("Octal Input: {}\n\n", octal));
    response.push_str(&format!("Binary Representation: {}\n", binary));
    response.push_str(&format!(
        "Hexadecimal Representation: {}\n",
        &binary_to_hex(&binary)?
    ));

    response.push_str(&format!("Decimal:\n{}", &binary_to_decimal(&binary)?));
    Ok(response)
}
fn build_decimal_respose(decimal: &str) -> Result<String, std::num::ParseIntError> {
    let mut response = String::new();
    let num: i128 = decimal.parse()?;
    let unsigned = decimal_to_unsigned(&num);
    let two_comp = decimal_to_2s_complement(&num)?;
    response.push_str(&format!("Decimal Input: {}\n\n", decimal));
    response.push_str("Binary Representation:\n");
    if let Some(ref unsigned_value) = unsigned {
        response.push_str(&format!("  Unsigned: {}\n", unsigned_value));
    }
    response.push_str(&format!("  2's Compliment: {}\n", two_comp));

    response.push_str("Hex Representation:\n");
    if let Some(ref unsigned_value) = unsigned {
        response.push_str(&format!("  Unsigned: {}\n", binary_to_hex(unsigned_value)?));
    }
    response.push_str(&format!("  2's Compliment: {}\n", binary_to_hex(&two_comp)?));

    response.push_str("Octal Representation:\n");
    if let Some(ref unsigned_value) = unsigned {
        response.push_str(&format!("  Unsigned: {}\n", binary_to_octal(unsigned_value)?));
    }
    response.push_str(&format!("  2's Compliment: {}\n", binary_to_octal(&two_comp)?));
   

    Ok(response)
}
fn parse_number(input: &str) -> Result<String, NumberError> {
    match input {
        s if s.starts_with("0x") => {
            let hex_part = &s[2..];
            let invalid_pos = find_invalid_positions(hex_part, |c| c.is_ascii_hexdigit());
            if invalid_pos.is_empty() {
                let response = build_hex_respose(&hex_part.to_uppercase())?;
                Ok(response)
            } else {
                Err(NumberError::InvalidChars(
                    s.to_string(),
                    invalid_pos.iter().map(|&i| i + 2).collect(),
                ))
            }
        }
        s if s.starts_with("0b") => {
            let bin_part = &s[2..];
            let invalid_pos = find_invalid_positions(bin_part, |c| c == '0' || c == '1');
            if invalid_pos.is_empty() {
                let response = build_binary_respose(bin_part)?;
                Ok(response)
            } else {
                Err(NumberError::InvalidChars(
                    s.to_string(),
                    invalid_pos.iter().map(|&i| i + 2).collect(),
                ))
            }
        }
        s if s.starts_with("0o") => {
            let oct_part = &s[2..];
            let invalid_pos = find_invalid_positions(oct_part, |c| c.is_digit(8));
            if invalid_pos.is_empty() {
                let response = build_octal_respose(oct_part)?;
                Ok(response)
            } else {
                Err(NumberError::InvalidChars(
                    s.to_string(),
                    invalid_pos.iter().map(|&i| i + 2).collect(),
                ))
            }
        }
        s if s.parse::<i64>().is_ok() => {
            let respose = build_decimal_respose(s)?;
            Ok(respose)
        }
        s => Err(NumberError::InvalidFormat(s.to_string())),
    }
}

fn create_error_display(input: &str, positions: &[usize]) -> String {
    let mut error_indicator = " ".repeat(input.len());
    for &pos in positions {
        if pos < error_indicator.len() {
            error_indicator.replace_range(pos..pos + 1, "^");
        }
    }
    format!("{}\n{}", input, error_indicator)
}

fn main() {
    let cmd = command!().arg(arg!([input] "Value to convert base 2: 0b, base 8: 0o, base 16: 0x, base 10 enter normal.").required(true));

    let matches = cmd.clone().get_matches();
    let input = matches.get_one::<String>("input").unwrap();
    match parse_number(input) {
        Ok(result) => println!("{}", result),
        Err(NumberError::InvalidChars(val, positions)) => {
            let error_display = create_error_display(&val, &positions);
            let mut cmd = cmd.clone();
            cmd.error(
                ErrorKind::InvalidValue,
                format!("Invalid characters at marked positions:\n{}", error_display),
            )
            .exit();
        }
        Err(NumberError::InvalidFormat(val)) => {
            let mut cmd = cmd.clone();
            cmd.error(
                ErrorKind::InvalidValue,
                format!("Unknown number format: {}. Expected format: 0d for decimal, 0x for hex, 0b for binary, or 0o for octal", val),
            )
            .exit();
        }
    }
}
