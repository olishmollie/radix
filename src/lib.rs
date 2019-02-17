pub struct Config {
    argument: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments.");
        }

        let argument = args[1].clone();

        Ok(Config { argument })
    }
}

pub fn run(config: Config) {
    let result = match &config.argument[0..2] {
        "0b" => binary_to_decimal(&config.argument),
        "0o" => octal_to_decimal(&config.argument),
        _ => {
            decimal_to_binary(&config.argument)
        }
    };

    match result {
        Ok(s) => println!("{}", s),
        Err(msg) => print_error(msg)
    };
}

pub fn print_error(msg: &str) {
    eprintln!("error: {}", msg);
}

pub fn print_usage() {
    eprintln!("Usage: dconv [options] <value>");
}

pub fn decimal_to_binary(argument: &str) -> Result<String, &'static str> {
    decimal_to_radix(2, argument, "0b")
}

pub fn decimal_to_octal(argument: &str) -> Result<String, &'static str> {
    decimal_to_radix(8, argument, "0o")
}

pub fn binary_to_decimal(argument: &str) -> Result<String, &'static str> {
    radix_to_decimal(2, argument)
}

pub fn octal_to_decimal(argument: &str) -> Result<String, &'static str> {
    radix_to_decimal(8, argument)
}

fn radix_to_decimal(radix: u32, argument: &str) -> Result<String, &'static str> {
    if radix < 2 || radix > 9 {
        panic!("invalid radix passed to radix_to_decimal()");
    }

    let mut result = 0;
    let mut power = 0; let mut err = false;

    for c in argument[2..].chars().rev() {
        match c.to_digit(10) {
            Some(digit) => {
                if digit > radix {
                    return Err("invalid conversion argument.");
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
        Err("invalid conversion argument.")
    } else {
        Ok(result.to_string())
    }
}

fn decimal_to_radix(radix: u32, argument: &str, prefix: &str) -> Result<String, &'static str> {
    if radix < 2 || radix > 9 {
        panic!("invalid radix passed to decimal_to_radix()");
    }

    let mut result = String::from(prefix);
    let mut number: i64;

    match argument.parse() {
        Ok(i) => number = i,
        Err(_) => return Err("invalid conversion argument.")
    };

    while number > 0 {
        let digit = number % radix as i64;
        result.insert_str(2, &digit.to_string());
        number /= radix as i64;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_decimal_to_binary() {
        assert_eq!(decimal_to_binary("42"), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_decimal_to_octal() {
        assert_eq!(decimal_to_octal("42"), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_binary_to_decimal() {
        assert_eq!(binary_to_decimal("0b101010"), Ok(String::from("42")));
    }

    #[test]
    fn converts_octal_to_decimal() {
        assert_eq!(octal_to_decimal("0o52"), Ok(String::from("42")));
    }

    #[test]
    fn does_not_convert_invalid_argument() {
        assert_eq!(decimal_to_binary("abcd"), Err("invalid conversion argument."));
        assert_eq!(decimal_to_octal("abcd"), Err("invalid conversion argument."));
        assert_eq!(binary_to_decimal("abcd"), Err("invalid conversion argument."));
        assert_eq!(octal_to_decimal("abcd"), Err("invalid conversion argument."));
    }

}
