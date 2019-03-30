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

    println!("You entered {}", degrees);
}
