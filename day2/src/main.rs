use std::fs;

// Rules:
// Is increasing or descreasing order
// Any two numbers adjacent shall differ no less than 1 and no more than 3

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Increasing,
    Decreasing,
}

// Check if the report is safe according to the rules
fn is_safe_report(report: &Vec<i32>) -> bool {
    // First check if it's safe without removing any numbers
    if check_report_safety(report) {
        return true;
    }

    // Try removing each number one at a time
    for i in 0..report.len() {
        let mut modified_report: Vec<i32> = report.clone();
        modified_report.remove(i);

        if check_report_safety(&modified_report) {
            return true;
        }
    }

    false
}

// Renamed original safety check function
fn check_report_safety(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut dir: Option<Direction> = None;

    for i in 0..report.len() - 1 {
        // Determine the direction of the report
        let mut new_dir: Option<Direction> = dir;

        if report[i] < report[i + 1] {
            new_dir = Some(Direction::Increasing);
        }

        if report[i] > report[i + 1] {
            new_dir = Some(Direction::Decreasing);
        }

        if i != 0 && dir != new_dir {
            return false;
        }

        dir = new_dir;

        // Check if the difference between adjacent numbers is between 1 and 3
        if (report[i] - report[i + 1]).abs() > 3 || (report[i] - report[i + 1]).abs() < 1 {
            return false;
        }
    }

    true
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Failed to read file!");
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut safe_count: i32 = 0;

    for line in contents.lines() {
        reports.push(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }

    for report in reports {
        if is_safe_report(&report) {
            safe_count += 1;
        }
    }

    println!("Safe Reports: {}", safe_count);
}
