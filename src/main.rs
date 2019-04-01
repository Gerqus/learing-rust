use std::io;

fn main() {
    let which_fibonacci_number: u8 = loop {
        let mut which_fibonacci_input = String::new();
        println!("Which subsequent Fibonacci number from 1 to 186 do you want to know?");

        io::stdin().read_line(&mut which_fibonacci_input)
            .expect("Couldn't read from stdin");

        match which_fibonacci_input.trim().parse() {
            Ok(n) => {
                if n <= 186 {
                    break n;
                } else {
                    continue;
                }
            },
            Err(_) => continue,
        };
    };

    println!("{}th Fibonacci number is equal to {}", which_fibonacci_number, get_fibonacci(which_fibonacci_number));
}

fn get_fibonacci(count: u8) -> u128 {
    let mut previous: u128 = 0;
    let mut current: u128 = 1;
    let mut tmp: u128;
    let mut i: u8 = 1;

    while i < count {
        tmp = previous;
        previous = current;
        current = current + tmp;
        i = i + 1;
    }

    current
}
