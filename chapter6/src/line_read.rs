use std::io::{BufRead, BufReader, Read};

pub fn get_lines<T: Read>(f: T) -> Vec<String> {
    let f = BufReader::new(f);

    let mut lines = Vec::new();

    for ll in f.lines() {
        lines.push(ll.unwrap());
    }
    lines
}
