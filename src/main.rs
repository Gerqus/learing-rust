use std::io;

fn main() {
    let which_fibonacci_number: u8 = loop {
        let mut which_fibonacci_input = String::new();
        println!("Which subsequent Fibonacci number do you want to know?");

        io::stdin().read_line(&mut which_fibonacci_input)
            .expect("Couldn't read from stdin");

        match which_fibonacci_input.trim().parse() {
            Ok(n) => {
                break n
            },
            Err(_) => continue
        };
    };

    println!("{}th Fibonacci number is equal to {}", which_fibonacci_number, get_fibonacci(which_fibonacci_number));
}

fn get_fibonacci(count: u8) -> u64 {
    if count == 0 {
        0
    } else if count == 1 {
        1
    } else {
        get_fibonacci(count - 2) + get_fibonacci(count -1)
    }
}
