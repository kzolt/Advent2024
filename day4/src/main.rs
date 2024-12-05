use std::fs;

const MAS: &[u8; 3] = &[b'M', b'A', b'S'];
const SAM: &[u8; 3] = &[b'S', b'A', b'M'];
const LF: &u8 = &b'\n';

fn diagonal(i: usize, v: &[u8], cols: usize) -> bool {
    let word_south_east: &[u8; 3] = match (v.get(i), v.get(i + cols + 1), v.get(i + cols * 2 + 2)) {
        (Some(b1), Some(b2), Some(b3)) => &[*b1, *b2, *b3],
        _ => return false,
    };

    let word_north_east: &[u8; 3] = match (v.get(i + cols * 2), v.get(i + cols + 1), v.get(i + 2)) {
        (Some(b1), Some(b2), Some(b3)) => &[*b1, *b2, *b3],
        _ => return false,
    };

    match (word_south_east, word_north_east) {
        (MAS, MAS) => true,
        (MAS, SAM) => true,
        (SAM, MAS) => true,
        (SAM, SAM) => true,
        _ => false,
    }
}
fn main() {
    let contents: Vec<u8> = fs::read("input.txt").unwrap();
    let mut count: usize = 0;

    let mut bytes = contents.iter();
    let mut cols: usize = 0;
    while bytes.next().unwrap() != LF {
        cols += 1;
    }
    cols += 1;

    for i in 0..contents.len() {
        if diagonal(i, &contents, cols) {
            count += 1;
        }
    }

    println!("Total matches found: {}", count);
}
