use std::io;

fn main() {
    let mut which_fibonacci_number: u16;

    loop {
        let mut which_fibonacci_input = String::new();
        println!("Which subsequent Fibonacci number do you want to know?");

        io::stdin().read_line(&mut which_fibonacci_input)
            .expect("Couldn't read from stdin");

        which_fibonacci_number = match which_fibonacci_input.trim().parse() {
            Ok(n) => {
                break n
            },
            Err(_) => continue
        };
    };

    println!("{}th Fibonacci number is equal to {}", which_fibonacci_number, getFibonacci(which_fibonacci_number));
}

fn getFibonacci(count: u16) -> u32 {
    if count == 0 {
        0
    } else if count == 1 {
        1
    } else {
        getFibonacci(count - 2) + getFibonacci(count -1)
    }
}
