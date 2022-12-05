use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_file = File::open("./input.txt").unwrap();
    let input_file_reader = BufReader::new(input_file);
    
    let mut total_sum = 0;
    for line in input_file_reader.lines() {
        let line = line.unwrap();
        let jobs = get_elves_jobs(&line);
        if jobs_contains_overlap(&jobs) {
            total_sum += 1;
        }
    }

    println!("Total overlapped jobs: {}", total_sum);
}

fn get_elves_jobs(line: &str) -> ((i32, i32),(i32, i32)) {
    let (elf_job_one, elf_job_two) = line.split_once(',').unwrap();

    let (elf_job_one_start_str, elf_job_one_end_str) = elf_job_one.split_once('-').unwrap();
    let elf_one_jobs = (elf_job_one_start_str.parse::<i32>().unwrap(), elf_job_one_end_str.parse::<i32>().unwrap());

    let (elf_job_two_start_str, elf_job_two_end_str) = elf_job_two.split_once('-').unwrap();
    let elf_two_jobs = (elf_job_two_start_str.parse::<i32>().unwrap(), elf_job_two_end_str.parse::<i32>().unwrap());

    (elf_one_jobs, elf_two_jobs)
}

fn jobs_contains_overlap(jobs: &((i32, i32), (i32, i32))) -> bool {
    let mut jobs = jobs.clone();

    match jobs {
        ((elf_one_start, elf_one_end), (elf_two_start, elf_two_end))
            // 2-8,3-7
            // 23-69,23-98
            if 
                (elf_one_start <= elf_two_start && 
                elf_one_end >= elf_two_end) ||
                (elf_one_start >= elf_two_start && 
                elf_one_end <= elf_two_end)
                => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    mod part_one {
        use crate::{get_elves_jobs, jobs_contains_overlap};

        #[test]
        fn example_one() {
            let line = "2-4,6-8";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs, ((2, 4), (6, 8)));
            assert_eq!(jobs_contains_overlap(&jobs), false);

            let line = "6-8,2-4";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs, ((6, 8), (2, 4)));
            assert_eq!(jobs_contains_overlap(&jobs), false);
        }
        #[test]
        fn example_two() {
            let line = "2-3,4-5";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);

            let line = "4-5,2-3";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);
        }
        #[test]
        fn example_three() {
            let line = "5-7,7-9";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);

            let line = "7-9,5-7";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);
        }
        #[test]
        fn example_four() {
            let line = "2-8,3-7";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs, ((2, 8), (3, 7)));
            assert_eq!(jobs_contains_overlap(&jobs), true);

            let line = "3-7,2-8";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs, ((3, 7), (2, 8)));
            assert_eq!(jobs_contains_overlap(&jobs), true);
        }
        #[test]
        fn example_five() {
            let line = "6-6,4-6";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), true);

            let line = "4-6,6-6";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), true);
        }
        #[test]
        fn example_six() {
            let line = "2-6,4-8";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);

            let line = "4-8,2-6";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), false);
        }
        #[test]
        fn example_from_file() {
            let line = "23-69,23-98";
            let jobs = get_elves_jobs(line);
            assert_eq!(jobs_contains_overlap(&jobs), true);
        }
    }
}
