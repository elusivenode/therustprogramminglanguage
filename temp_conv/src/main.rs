mod temp_conv;
use std::env;
use temp_conv::temp_conv;

fn main() {
    // to call: ./target/debug/temp_conv 50 C
    // or:      cargo run 50 c
    let args: Vec<String> = env::args().collect();
    let temp: f64 = args[1].parse::<f64>().unwrap();
    let scale: String = args[2].clone();
    let (temp, scale) = temp_conv(temp, &scale);
    println!("converted temp is {temp:.2}{scale}");
    let a = (temp * 100.0).round() / 100.0;
    println!("{a}");
    let (temp, scale) = temp_conv(temp, &scale);
    println!("converted back temp is {temp:.2}{scale}");
}
