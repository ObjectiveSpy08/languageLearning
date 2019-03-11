use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter the number: ");
    io::stdin().read_line(&mut n).expect("Error while reading");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("Factorial of {} is {}", n, factorial1(n));
    println!("Not working Factorial of {} is {}", n, factorial2(n));
}

fn factorial1(n: u32) -> u32 {
    // With mutable
    let mut fact = 1;
    for x in 2..n + 1 {
        fact = fact * x;
    }
    fact
}

fn factorial2(n: u32) -> u32 {
    // With shadowing
    // Not working for some reason. TBD: See why
    let fact = 1;
    for x in 2..n + 1 {
        let fact = fact * x;
        println!("{}", fact);
    }
    fact
}
