use std::io;

fn main() {
    loop {
        let mut temperature = String::new();

        println!("Type the temperature in Fahrenheit.");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read input.");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Not a valid number.");
                continue;
            }
        };

        let celcius: f32 = (temperature - 32.0) * 5.0 / 9.0;

        println!("Temperature is Celsius is: {celcius}C degrees.");
        break;
    }
}
