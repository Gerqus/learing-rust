use std::io;

fn main() {
    let degrees: f32 = loop {
        let mut degrees = String::new();
        
        println!("Please enter number of degrees");

        io::stdin().read_line(&mut degrees)
            .expect("Could not read input");

        match degrees.trim().parse() {
            Ok(number) => break number,
            Err(_) => continue
        }
    };

    let unit: char = loop {
        let mut unit = String::new();
        
        println!("Please enter input degrees unit: c - Celsius or f - Farenheit");

        io::stdin().read_line(&mut unit)
            .expect("Could not read input");
        let unit = unit.trim().chars().next().unwrap();
        
        if unit == 'c' || unit == 'f' {
            break unit;
        }
    };

    match unit {
        'c' => {
            println!("You entered {}*C", degrees);
            println!("It is {}F", convert_to_fahrenheit(degrees));
        },
        'f' => {
            println!("You entered {}F", degrees);
            println!("It is {}*C", convert_to_celsius(degrees));
        },
        _ => panic!("Wrong unit after check. Weird stuff happens... o.O"),
    };
}

fn convert_to_fahrenheit(degrees: f32) -> f32 {
    degrees * 1.8 + 32.0
}

fn convert_to_celsius(degrees: f32) -> f32 {
    (degrees - 32.0) / 1.8
}