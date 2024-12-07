use std::{collections::HashMap, fs};

fn get_rules(contents: &str) -> HashMap<u8, Vec<u8>> {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    for line in contents.lines() {
        let mut split = line.split("|");

        let key: u8 = split.next().unwrap().parse::<u8>().unwrap();
        let value: u8 = split.next().unwrap().parse::<u8>().unwrap();

        rules
            .entry(key)
            .and_modify(|key| key.push(value))
            .or_insert(Vec::from([value]));
    }

    rules
}

fn get_page_updates(contents: &str) -> Vec<u8> {
    let mut split = contents.split(',');
    let mut updates: Vec<u8> = Vec::new();

    while let Some(value) = split.next() {
        updates.push(value.parse::<u8>().unwrap());
    }

    updates
}

fn is_ordered(updates: &Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 1..updates.len() {
        let (page, next_page) = (updates[i - 1], updates[i]);

        if let Some(pages) = rules.get(&page) {
            if !pages.contains(&next_page) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let mut split = contents.split("\n\n");
    let mut res: i64 = 0;

    let page_rules = split.next().unwrap();
    let page_updates = split.next().unwrap();

    let rules: HashMap<u8, Vec<u8>> = get_rules(&page_rules);

    let mut unordered_updates: Vec<Vec<u8>> = Vec::new();
    for line in page_updates.lines() {
        let updates: Vec<u8> = get_page_updates(line);

        if !is_ordered(&updates, &rules) {
            unordered_updates.push(updates.clone());
        }
    }

    for updates in unordered_updates.iter_mut() {
        for i in 0..updates.len() - 1 {
            for j in i + 1..updates.len() {
                let (page, next_page) = (updates[i], updates[j]);

                let mut ordered: bool = true;
                if let Some(pages) = rules.get(&page) {
                    if !pages.contains(&next_page) {
                        ordered = false;
                    }
                } else {
                    ordered = false;
                }

                if !ordered {
                    (updates[i], updates[j]) = (next_page, page);
                }
            }
        }

        res += updates[updates.len() / 2] as i64;
    }

    println!("{}", res);
}
