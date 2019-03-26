use std::io::{self, Write};
use std::fmt;


const CONVERSION_COEFFICIENT: f64 = 9.0/5.0;
const CONVERSION_CONSTANT: f64 = 32.0;


fn main() {
    print!("Enter your temperature to convert: ");
    io::stdout().flush().expect("Unable to flush to std out");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let input = input.trim();

    let (temperature_string, to_temperature): (&str, &Fn(f64) -> Temperature) = match input.chars().last().expect("The input was empty") {
        'c' | 'C' => (&(input[..input.len()-1]), &Temperature::Celsius),
        'f' | 'F' => (&(input[..input.len()-1]), &Temperature::Farenheit),
        _ => (input, &Temperature::Farenheit)
    } ;
    
    println!("Conversion is {}", build_temp(temperature_string, to_temperature).convert());
}

fn build_temp(input: &str, to_temperature: &Fn(f64) -> Temperature) -> Temperature {
    let temp = parse_temp(input.trim().to_string());
    return to_temperature(temp);
}

fn parse_temp(input: String) -> f64 {
    let float = input.parse::<f64>();
    let int = input.parse::<i32>();
    match (float, int) {
        (Ok(f), _) => f,
        (_, Ok(i)) => i as f64,
        (Err(e1), Err(e2)) => panic!("Could not parse {} as floating point number, {}, {}", input, e1, e2)
    }
}

enum Temperature {
    Farenheit(f64),
    Celsius(f64),
}

impl Temperature {
    fn convert(self) -> Temperature {
        return match self {
            Temperature::Farenheit(f) => Temperature::Celsius((f - CONVERSION_CONSTANT) / CONVERSION_COEFFICIENT),
            Temperature::Celsius(c) => Temperature::Farenheit(c * CONVERSION_COEFFICIENT + CONVERSION_CONSTANT)
        }; 
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (temp, unit) = match self {
            Temperature::Farenheit(t) => (t, "F"),
            Temperature::Celsius(t) => (t, "C")
        };

        return write!(f, "{:.2} {}", temp, unit);

    }

}