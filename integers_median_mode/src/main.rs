use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Replace "your_file.txt" with the actual file path
    let file_path =
        "/Users/hamish.macdonald/DEv/therustprogramminglanguage/integers_median_mode/assets/integers.txt";

    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut integers: HashMap<i32, i32> = HashMap::new();
    let mut sorted_int_vec: Vec<i32> = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        if let Ok(number) = line?.parse::<i32>() {
            sorted_int_vec.push(number);
            sorted_int_vec.sort(); // for median calc
            let ct = integers.entry(number).or_insert(0); // for mode calc
            *ct += 1;
        } else {
            eprintln!("Error: Invalid integer format in the file.");
        }
    }

    for v in sorted_int_vec.iter() {
        print!("{} ", v);
    }

    let mode = get_mode(&integers);
    println!("\nmode values:");
    for i in mode.iter() {
        println!("  {}", i);
    }

    let median = get_median(&sorted_int_vec);
    println!("\nmedian value:");
    println!("  {}", median);

    Ok(())
}

fn get_median(v: &Vec<i32>) -> f64 {
    let len = v.len();
    let median;
    if len % 2 != 0 {
        let idx = len / 2;
        median = v[idx].try_into().unwrap();
    } else {
        let idx = len / 2;
        median = ((v[idx - 1] + v[idx]) / 2).try_into().unwrap();
    }
    median
}

fn get_mode(hm: &HashMap<i32, i32>) -> Vec<i32> {
    let mut mode: Vec<i32> = Vec::new();
    let mut max_value = 0;

    for (key, value) in hm.iter() {
        if *value > max_value {
            mode.clear();
            max_value = *value;
            mode.push(*key);
            continue;
        }
        if *value == max_value {
            mode.push(*key);
        }
    }
    mode
}
