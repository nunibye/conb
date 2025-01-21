pub fn hex_to_binary(hex: &str) -> Result<String, std::num::ParseIntError> {
    let num = u128::from_str_radix(hex, 16)?;
    Ok(format!("{:b}", num))
}

pub fn octal_to_binary(octal: &str) -> Result<String, std::num::ParseIntError> {
    let num = u128::from_str_radix(octal, 8)?;
    Ok(format!("{:b}", num))
}

pub fn binary_to_hex(binary: &str) -> Result<String, std::num::ParseIntError> {
    let num = u128::from_str_radix(binary, 2)?;
    Ok(format!("{:X}", num))
}

pub fn binary_to_octal(binary: &str) -> Result<String, std::num::ParseIntError> {
    let num = u128::from_str_radix(binary, 2)?;
    Ok(format!("{:o}", num))
}

fn get_bit_length(num: i128) -> usize {
    let mut length: usize = 1;
    while num < (-(1 << (length - 1))) {
        length += 1;
    }
    return length;
}

pub fn decimal_to_2s_complement(decimal: &i128) -> Result<String, std::num::ParseIntError> {
    let num = *decimal;
    if num < 0 {
        let inverted = !num;
        let twos_complement = -(inverted as i128 + 1);
        let length = get_bit_length(num);
        let binary_string = format!("{:b}", twos_complement);
        let truncated_string: String = binary_string
            .chars()
            .rev()
            .take(length)
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        Ok(truncated_string)
    } else {
        Ok(format!("0{:b}", num))
    }
}

pub fn decimal_to_unsigned(decimal: &i128) -> Option<String> {
    let num = *decimal;
    if num < 0 {
        return None;
    }else {
        Some(format!("{:b}", num))
    }

}

pub fn binary_to_decimal(binary: &str) -> Result<String, std::num::ParseIntError> {
    let bit_length = binary.len();
    let unsigned = u128::from_str_radix(binary, 2)?;
    let mask = (1 << bit_length) - 1;
    let twos_complement = if binary.starts_with('1') {
        let inverted = !unsigned & mask;
        -(inverted as i128 + 1)
    } else {
        unsigned as i128
    };
    let inverted = !unsigned & mask;
    let ones_complement: i128 = if binary.starts_with('1') {
        -(inverted as i128)
    } else {
        inverted as i128
    };
    Ok(format!(
        "  Unsigned: {}\n  2's Compliment: {}\n  1's Compliment: {}\n",
        unsigned, twos_complement, ones_complement
    ))
}
