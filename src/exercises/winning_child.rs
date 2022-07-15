use std::io::{self, Write};

fn max(arg: &[u32]) -> u32 {
    let mut max = 0;

    arg.iter().for_each(|&x| {
        if x > max {
            max = x;
        }
    });

    max
}

fn find_index(number: u32, array: &[u32]) -> usize {
    let index = array.iter().position(|&x| x == number).unwrap();
    index
}

pub fn winning_child() -> () {
    let mut length: String = String::new();
    let mut vec_numbers: Vec<u32> = Vec::new();
    let mut vec_names: Vec<String> = Vec::new();

    print!("Enter the length of the array: ");
    io::stdout().flush().expect("Could not flush stdout");

    io::stdin()
        .read_line(&mut length)
        .expect("Failed to read line");

    let length: u32 = length.trim().parse().expect("Please type a number");

    for _ in 0..length {
        let mut number: String = String::new();

        print!("Enter a number: ");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: u32 = number.trim().parse().expect("Please type a number");

        vec_numbers.push(number);
    }

    for _ in 0..length {
        let mut name = String::new();

        print!("Enter name: ");
        io::stdout().flush().expect("Could not flush stdout");

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        vec_names.push(name);
    }

    let max_number = max(&vec_numbers);
    let index = find_index(max_number, &vec_numbers);

    println!("{} {} {}", index, vec_names[index], max_number);
}
