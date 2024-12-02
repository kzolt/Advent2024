use std::fs;
use std::collections::HashMap;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file"); 

    // Split the input into lines
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let mut right_hash: HashMap<i32, i32> = HashMap::new();

   for line in input.lines() {
        let (left, right) = line.split_once("   ").expect("Failed to split line");

        left_list.push(left.parse().expect("Failed to parse left value"));
        right_list.push(right.parse().expect("Failed to parse right value"));

        *right_hash.entry(right.parse().expect("Failed to parse right value")).or_insert(0) += 1;
    }

    // Sort the lists
    left_list.sort();
    right_list.sort();

    // Calculate the sum of the differences and the simularity score
    let mut sum = 0;
    let mut simularity_score = 0;
    for i in 0..left_list.len() {
        sum += (left_list[i] - right_list[i]).abs();
        simularity_score += left_list[i] * right_hash.get(&left_list[i]).unwrap_or(&0);
    }

    println!("Sum: {}", sum);
    println!("Simularity score: {}", simularity_score);
}
