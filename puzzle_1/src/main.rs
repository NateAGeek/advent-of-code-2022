use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let input_file = File::open("./input.txt").unwrap();
    let input_file_reader = BufReader::new(input_file);

    let mut highest_calorie_count = 0;
    let mut active_sum = 0;
    for line in input_file_reader.lines() {
        let line = line.unwrap();
        if line == "" {
            println!("End of Elf Inventory, Total Cal: {}", active_sum);
            if active_sum > highest_calorie_count {
                highest_calorie_count = active_sum;
            }
            active_sum = 0;
        } else {
            active_sum += line.parse::<i32>().unwrap();
        }
    }

    println!("The Highest Calorie Count is: {}", highest_calorie_count);
}
