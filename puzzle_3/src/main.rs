use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_file = File::open("./input.txt").unwrap();
    let input_file_reader = BufReader::new(input_file);

    let mut total_sum = 0;
    for line in input_file_reader.lines() {
        let line = line.unwrap();
        let (first_compartment, second_compartment) = line.split_at(line.len()/2);
        let item = find_match_item_in_compartments(first_compartment, second_compartment);
        let item_value = item_value(item);

        total_sum += item_value;
    }

    println!("Total Sum For Items: {}", total_sum);
}

fn find_match_item_in_compartments(first_compartment: &str, second_compartment: &str) -> char {
    let mut item_count_hash: HashMap<char, i8> = HashMap::new();

    for char in first_compartment.chars() {
        if item_count_hash.get(&char).is_none() {
            item_count_hash.insert(char, 1);
        }
    }
    for char in second_compartment.chars() {
        let char = item_count_hash.get_mut(&char);
        if let Some(entry) = char {
            if entry < &mut 2 {
                *entry += 1;
            }
        }
    }

    let max_entry = item_count_hash
    .iter()
    .max_by_key(|entry| entry.1)
    .unwrap();

    return max_entry.0.clone();
}

fn item_value(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - 96 // convert ascii a-z (97 - 122) to range (1-26)
    } else if item.is_ascii_uppercase() {
        (item as u32 - 65) + 27 // convert ascii a-z (65 - 122) to range (1-26)
    } else {
        0 //Not valid? Should probably throw an err
    }
}

#[cfg(test)]
mod tests {

    mod examples_part_one {
        use crate::{find_match_item_in_compartments, item_value};

        
        #[test]
        fn example_one() {
            let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
            let (first_compartment, second_compartment) = line.split_at(line.len()/2);
            assert_eq!("vJrwpWtwJgWr", first_compartment);
            assert_eq!("hcsFMMfFFhFp", second_compartment);
            assert_eq!('p', find_match_item_in_compartments(first_compartment, second_compartment));
            assert_eq!(16, item_value('p'));
        }
        #[test]
        fn example_two() {
            let line = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
            let (first_compartment, second_compartment) = line.split_at(line.len()/2);
            assert_eq!("jqHRNqRjqzjGDLGL", first_compartment);
            assert_eq!("rsFMfFZSrLrFZsSL", second_compartment);
            assert_eq!('L', find_match_item_in_compartments(first_compartment, second_compartment));
            assert_eq!(38, item_value('L'));
        }
        #[test]
        fn example_three() {
            let line = "PmmdzqPrVvPwwTWBwg";
            let (first_compartment, second_compartment) = line.split_at(line.len()/2);
            assert_eq!("PmmdzqPrV", first_compartment);
            assert_eq!("vPwwTWBwg", second_compartment);
            assert_eq!('P', find_match_item_in_compartments(first_compartment, second_compartment));
            assert_eq!(42, item_value('P'));
        }
    }
}