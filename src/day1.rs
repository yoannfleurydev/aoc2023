use std::fs;

pub fn init(file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sentences = contents.split("\n");

    let sum: i32 = sentences
        .into_iter()
        .map(|sentence| {
            let first = sentence
                .chars()
                .into_iter()
                .find(|ch| ch.to_digit(10).is_some())
                .unwrap_or('0');
            let last = sentence
                .chars()
                .into_iter()
                .rev()
                .find(|ch| ch.to_digit(10).is_some())
                .unwrap_or('0');

            let mut concat = String::from(first);
            concat.push(last);

            match concat.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            }
        })
        .sum();

    println!("{:?}", sum)
}
