use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn setup() -> Vec<String> {
    let mut urls = Vec::new();
    if let Ok(lines) = read_lines("./data/urls.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(url) = line {
                urls.push(url)
            }
        }
    } else {
        panic!("Failed to open");
    }
    // println!();
    // print!("Next batch: ");
    urls
}
