use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter the number: ");
    io::stdin()
        .read_line(&mut n)
        . expect("Error while reading");
    let n: i32 = n.trim().parse().expect("Error while casting to number");
    println!("{} Fibonacci numbers: ", n);
    fibo(n);
}
fn fibo(n: i32) {
    let (mut a, mut b) = (-1, 1);
    // Since c isn't used outside, no need to declare here
    // Initially, was trying to declare it inside the tuple. But initilizing it was giving warning that initial value isn't used as we overried inside the for.
    for _ in 1..n+1 {
        let c = a + b;
        a = b;
        b = c;
        print!("{} ", c);
    }
    println!("");
}