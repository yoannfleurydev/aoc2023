use regex::Regex;
use std::fs;

pub fn init(file_path: &String) {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"(?<number>\d+)").expect("Cannot create regex");
    content.lines().for_each(|line| {
        let mut locs = re.capture_locations();
        re.captures_read(&mut locs, line);

        println!("{:?}", locs)
    });
}
