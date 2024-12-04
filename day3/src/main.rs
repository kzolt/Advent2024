use regex::Regex;
use std::fs;

fn get_start_index(index: usize, do_matches: &Vec<regex::Match>) -> usize {
    if index == 0 {
        0
    } else {
        do_matches[index - 1].end()
    }
}

fn get_end_index(
    index: usize,
    start_index: usize,
    dont_matches: &Vec<regex::Match>,
    total_length: usize,
) -> usize {
    let mut end_pointer: usize = index;

    if index == 0 {
        return dont_matches[end_pointer].end();
    }

    if end_pointer + 1 >= dont_matches.len() {
        return total_length;
    }

    while start_index > dont_matches[end_pointer].end() {
        end_pointer += 1;
    }

    dont_matches[end_pointer].end()
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Failed to read file");

    let search_pattern: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do_pattern: Regex = Regex::new(r"do\(\)").unwrap();
    let dont_pattern: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut sum: i32 = 0;

    // Collect matches into vectors first
    let do_matches: Vec<_> = do_pattern.find_iter(&contents).collect();
    let dont_matches: Vec<_> = dont_pattern.find_iter(&contents).collect();

    let mut previous_start_index: usize = 0;
    let mut previous_end_index: usize = 0;

    for i in 0..do_matches.len() {
        let start_index = get_start_index(i, &do_matches);
        let end_index = get_end_index(i, start_index, &dont_matches, contents.len());

        if i != 0 {
            if end_index == previous_end_index || start_index == previous_start_index {
                continue;
            }
        }

        previous_start_index = start_index;
        previous_end_index = end_index;

        println!("({}, {})", start_index, end_index);

        let substring: String = contents[start_index..end_index].to_string();

        for capture in search_pattern.captures_iter(&substring) {
            let x = &capture[1];
            let y = &capture[2];

            sum += x.parse::<i32>().expect("Failed to convert X value")
                * y.parse::<i32>().expect("Failed to convert Y value");
        }
    }

    println!("{}", sum);
}
