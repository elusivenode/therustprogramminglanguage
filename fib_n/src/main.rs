mod fib_n;
use fib_n::fib_n;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args[2].parse::<u64>().unwrap();
    let fib = fib_n(n);
    println!("The {n}th fibonacci number is {fib}");
}
