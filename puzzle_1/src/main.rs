use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let input_file = File::open("./input.txt").unwrap();
    let input_file_reader = BufReader::new(input_file);

    let mut calorie_counts: [i32; 3] = [0, 0, 0];
    let mut current_elf_calorie_sum = 0;
    let mut lowest_number_index: usize = 0;

    for line in input_file_reader.lines() {
        let line = line.unwrap();

        if line == "" {
            println!("End of Elf Inventory, Total Cal: {}", current_elf_calorie_sum);

            // THis finds the first smallest, not the smallest... Might just want to store the index of smallest...
            for index in 0..calorie_counts.len(){
                if current_elf_calorie_sum < calorie_counts[lowest_number_index] {
                    lowest_number_index = index;
                }
            }

            if calorie_counts[lowest_number_index] < current_elf_calorie_sum {
                calorie_counts[lowest_number_index] = current_elf_calorie_sum;
            }
            current_elf_calorie_sum = 0;
        } else {
            current_elf_calorie_sum += line.parse::<i32>().unwrap();
        }
    }

    println!("The Highest 3 Calorie Count is: {:?}", calorie_counts);
}
