use std::process;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage:
    nconv -h | --help
    nconv -v | --version
    nconv [-b | -d | -o | -x ] <value>

Options:
    -h, --help         Print this message.
    -v, --version      Print version.
    -b, --binary       Set target to binary.
    -d, --decimal      Set target to decimal.
    -o, --octal        Set target to octal.
    -x, --hexadecimal  Set target to hexadecimal.
";

enum Target {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

pub struct Config {
    value: String,
    target: Target,
}

impl Config {
    pub fn new(argv: &[String]) -> Result<Config, &'static str> {
        let args = Docopt::new(USAGE)
            .and_then(|d| d.argv(argv.into_iter()).parse())
            .unwrap_or_else(|e| e.exit());

        if args.get_bool("-v") {
            println!("nconv {}", VERSION);
            process::exit(0);
        }

        let value = args.get_str("<value>").to_string();

        let target: Target;
        if args.get_bool("-b") {
            target = Target::Binary;
        } else if args.get_bool("-o") {
            target = Target::Octal;
        } else if args.get_bool("-x") {
            target = Target::Hexadecimal;
        } else {
            target = Target::Decimal;
        }

        Ok(Config { value, target })
    }
}

pub fn run(config: Config) -> Result<String, &'static str> {
    if config.value.starts_with("0") && config.value.len() >= 2 {
        match &config.value[0..2] {
            "0b" => match config.target {
                Target::Binary => {
                    let tmp = binary_to_decimal(&config.value)?;
                    decimal_to_binary(&tmp)
                }
                Target::Decimal => binary_to_decimal(&config.value),
                Target::Hexadecimal => {
                    let tmp = binary_to_decimal(&config.value)?;
                    decimal_to_hexadecimal(&tmp)
                }
                Target::Octal => {
                    let tmp = binary_to_decimal(&config.value)?;
                    decimal_to_octal(&tmp)
                }
            },
            "0o" => match config.target {
                Target::Binary => {
                    let tmp = octal_to_decimal(&config.value)?;
                    decimal_to_binary(&tmp)
                }
                Target::Decimal => octal_to_decimal(&config.value),
                Target::Hexadecimal => {
                    let tmp = octal_to_decimal(&config.value)?;
                    decimal_to_hexadecimal(&tmp)
                }
                Target::Octal => {
                    let tmp = octal_to_decimal(&config.value)?;
                    decimal_to_octal(&tmp)
                }
            },
            "0x" => match config.target {
                Target::Binary => {
                    let tmp = hexadecimal_to_decimal(&config.value)?;
                    decimal_to_binary(&tmp)
                }
                Target::Decimal => hexadecimal_to_decimal(&config.value),
                Target::Hexadecimal => {
                    let tmp = hexadecimal_to_decimal(&config.value)?;
                    decimal_to_hexadecimal(&tmp)
                }
                Target::Octal => {
                    let tmp = hexadecimal_to_decimal(&config.value)?;
                    decimal_to_octal(&tmp)
                }
            },
            _ => Err("Invalid conversion value."),
        }
    } else {
        match config.target {
            Target::Binary => decimal_to_binary(&config.value),
            Target::Decimal => {
                let tmp = decimal_to_binary(&config.value)?;
                binary_to_decimal(&tmp)
            }
            Target::Octal => decimal_to_octal(&config.value),
            Target::Hexadecimal => decimal_to_hexadecimal(&config.value),
        }
    }
}

fn decimal_to_binary(value: &str) -> Result<String, &'static str> {
    decimal_to_radix(2, value, "0b")
}

fn decimal_to_octal(value: &str) -> Result<String, &'static str> {
    decimal_to_radix(8, value, "0o")
}

fn decimal_to_hexadecimal(value: &str) -> Result<String, &'static str> {
    decimal_to_radix(16, value, "0x")
}

fn binary_to_decimal(value: &str) -> Result<String, &'static str> {
    radix_to_decimal(2, value)
}

fn octal_to_decimal(value: &str) -> Result<String, &'static str> {
    radix_to_decimal(8, value)
}

fn hexadecimal_to_decimal(value: &str) -> Result<String, &'static str> {
    radix_to_decimal(16, value)
}

fn radix_to_decimal(radix: u32, value: &str) -> Result<String, &'static str> {
    let mut result = 0;
    let mut power = 0;

    for c in value[2..].chars().rev() {
        match to_number(c) {
            Ok(n) => {
                if n >= radix {
                    return Err("Invalid conversion value.");
                }
                result += n * radix.pow(power);
                power += 1;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(result.to_string())
}

fn decimal_to_radix(radix: u32, value: &str, prefix: &str) -> Result<String, &'static str> {
    let mut result = String::from(prefix);
    let mut number: u32;

    match value.parse() {
        Ok(i) => number = i,
        Err(_) => return Err("Invalid conversion value."),
    };

    while number > 0 {
        let digit = number % radix;
        match to_string(digit) {
            Ok(s) => result.insert_str(2, &s),
            Err(e) => return Err(e),
        }
        number /= radix;
    }

    Ok(result)
}

fn to_string(digit: u32) -> Result<String, &'static str> {
    if digit <= 9 {
        Ok(digit.to_string())
    } else {
        match digit {
            10 => Ok(String::from("a")),
            11 => Ok(String::from("b")),
            12 => Ok(String::from("c")),
            13 => Ok(String::from("d")),
            14 => Ok(String::from("e")),
            15 => Ok(String::from("f")),
            _ => Err("Invalid conversion value."),
        }
    }
}

fn to_number(c: char) -> Result<u32, &'static str> {
    match c.to_digit(10) {
        Some(n) => Ok(n),
        None => match c {
            'a' => Ok(10),
            'b' => Ok(11),
            'c' => Ok(12),
            'd' => Ok(13),
            'e' => Ok(14),
            'f' => Ok(15),
            _ => Err("Invalid conversion value."),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(mut args: Vec<&'static str>) -> Config {
        args.insert(0, "nconv");
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Config::new(&args).unwrap()
    }

    #[test]
    fn converts_decimal_to_binary() {
        let args = vec!["42", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_hexadecimal_to_binary() {
        let args = vec!["0x2a", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_octal_to_binary() {
        let args = vec!["0o52", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_decimal_to_octal() {
        let args = vec!["42", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_binary_to_octal() {
        let args = vec!["0b101010", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_hexadecimal_to_octal() {
        let args = vec!["0x2a", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_decimal_to_hexadecimal() {
        let args = vec!["42", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_binary_to_hexadecimal() {
        let args = vec!["0b101010", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_octal_to_hexadecimal() {
        let args = vec!["0o52", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_binary_to_decimal() {
        let args = vec!["0b101010"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn converts_hexadecimal_to_decimal() {
        let args = vec!["0x2a"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn converts_octal_to_decimal() {
        let args = vec!["0o52"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn parses_long_flags() {
        let args = vec!["0o52", "--binary"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn does_not_convert_invalid_argument() {
        let args = vec!["0h42"];
        let config = test_config(args);
        assert_eq!(run(config), Err("Invalid conversion value."));
    }

    #[test]
    fn converts_its_own_radix() {
        let args = vec!["42", "-d"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")))
    }

    #[test]
    fn converts_empty_argument() {
        let args = vec!["0b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0")));
    }

    #[test]
    fn converts_one_char_arguments() {
        let args = vec!["5", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101")));
    }

    #[test]
    fn does_not_convert_invalid_radix() {
        let args = vec!["0b12"];
        let config = test_config(args);
        assert_eq!(run(config), Err("Invalid conversion value."));
    }

}
