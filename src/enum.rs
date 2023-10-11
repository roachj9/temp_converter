use std::{io};


fn main() -> Result<(), &'static str> {

    println!("Convert temperature between Fahrenheit and Celsius");
    println!("For example, enter 25C or 78F and the temperature will get converted!");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let trimmed_input = input.trim();

    let temperature: f32;

    enum Unit {
        c = 'C',
        f = 'F',
    }

    if trimmed_input.ends_with(Unit::c)  {
        temperature = match trimmed_input[..trimmed_input.len() - 1].parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("Invalid input. Please enter a valid number followed by 'C' or 'F'.");
            }
        };
        unit = "C";
    } else if trimmed_input.ends_with(Unit::f) {
        temperature = match trimmed_input[..trimmed_input.len() - 1].parse() {
            Ok(num) => num,
            Err(_) => {
                
                return Err("Invalid input. Please enter a valid number followed by 'C' or 'F'.");
            }
        };
        unit = "F";
    } else {
        return Err("Invalid input. Please enter a valid number followed by 'C' or 'F'.");
    }


    
    println!("Inputted temperature:{}", input);
    match unit {
        Unit:c => {
            let convert_to_f = (temperature  * (9.0 / 5.0)) +32.0;

            println!("Temperature in Fahrenheit: {}°F", convert_to_f.round());
            return Ok(());
        }
        Unit:f => {
            let convert_to_c = (temperature - 32.0) * 5.0 / 9.0;

            println!("Temperature in Celcius: {}°C", convert_to_c.round());
            return Ok(());
        }
        _ => unreachable!(),
    } 
}
    