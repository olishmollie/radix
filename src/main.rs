mod conversions;

fn main() {
    let test1 = conversions::decimal_to_binary("10");
    let test2 = conversions::binary_to_decimal("1011");

    match test1 {
        Ok(i) => println!("{}", i),
        Err(msg) => println!("error: {}", msg)
    }

    match test2 {
        Ok(i) => println!("{}", i),
        Err(msg) => println!("error: {}", msg)
    }
}
