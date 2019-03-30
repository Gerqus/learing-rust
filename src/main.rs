use std::io;

fn main() {
    let degrees: i32 = loop {
        let mut degrees = String::new();
        
        println!("Please enter number of degrees");

        io::stdin().read_line(&mut degrees)
            .expect("Could not read input");

        match degrees.trim().parse() {
            Ok(number) => break number,
            Err(_) => continue
        }
    };

    let unit: String = loop {
        let mut unit = String::new();
        
        println!("Please enter input degrees unit: c - Celsius or f - Farenheit");

        io::stdin().read_line(&mut unit)
            .expect("Could not read input");
        unit = String::from(unit.trim());
        
        if unit == "c" || unit == "f" {
            break unit;
        }
    };

    println!("You entered {}*{}", degrees, unit);
}
