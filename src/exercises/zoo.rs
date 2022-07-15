use std::io::{self, Write};

fn unique_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    arr.iter().for_each(|&x| {
        if !result.contains(&x) {
            result.push(x);
        }
    });

    result
}

fn zoo() -> () {
    let mut length: String = String::new();
    let mut arr_numbers: String = String::new();

    print!("Enter the length of the array: ");
    io::stdout().flush().expect("Could not flush stdout");

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");

    print!("Enter array of numbers: ");
    io::stdout().flush().expect("Could not flush stdout");

    io::stdin()
        .read_line(&mut arr_numbers)
        .expect("Failed to read line");

    let arr_numbers = arr_numbers
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let arr_numbers = unique_numbers(&arr_numbers).sort();

    println!("{:?}", arr_numbers);
}
