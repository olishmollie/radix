pub fn decimal_to_binary(target: &str) -> Result<String, &'static str> {
    decimal_to_radix(2, target, "0b")
}

pub fn binary_to_decimal(target: &str) -> Result<String, &'static str> {
    radix_to_decimal(2, target)
}

pub fn hex_to_decimal(target: &str) -> Result<String, &'static str> {
    radix_to_decimal(6, target)
}

pub fn octal_to_decimal(target: &str) -> Result<String, &'static str> {
    radix_to_decimal(8, target)
}

fn radix_to_decimal(radix: u32, target: &str) -> Result<String, &'static str> {
    let mut result = 0;
    let mut power = 0;
    let mut err = false;

    for c in target.chars().rev() {
        match c.to_digit(10) {
            Some(digit) => {
                if digit > radix {
                    return Err("invalid conversion target.");
                }
                result += digit * radix.pow(power);
                power += 1;
            }
            None => {
                err = true;
                break;
            }
        }
    }

    if err {
        Err("invalid conversion target.")
    } else {
        Ok(result.to_string())
    }
}

fn decimal_to_radix(radix: u32, target: &str, prefix: &str) -> Result<String, &'static str> {
    let mut result = String::from(prefix);
    let mut number: i64;

    match target.parse() {
        Ok(i) => number = i,
        Err(_) => return Err("invalid conversion target.")
    };

    while number > 0 {
        let digit = number % radix as i64;
        result.insert_str(2, &digit.to_string());
        number /= 2;
    }

    Ok(result)
}