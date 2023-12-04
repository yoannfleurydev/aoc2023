use regex::Regex;
use std::fs;

pub fn init(file_path: &String) -> i32 {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"\d+").expect("Regex should create");
    content
        .lines()
        .map(|line| {
            let card = line.split(":").collect::<Vec<&str>>();
            let rest = card.get(1).unwrap();
            let numbers = rest.split("|").collect::<Vec<&str>>();
            let (winning, mine) = (numbers.get(0).unwrap(), numbers.get(1).unwrap());

            let winning_items = get_numbers(&re, &winning);
            let my_items = get_numbers(&re, &mine);

            winning_items
                .into_iter()
                .fold(0, |acc: i32, e| match my_items.contains(&e) {
                    true => {
                        if acc == 0 {
                            1
                        } else {
                            acc * 2
                        }
                    }
                    false => acc,
                })
        })
        .sum()
}

fn get_numbers<'a>(re: &'a Regex, sample: &'a str) -> Vec<&'a str> {
    let captures = re.captures_iter(&sample);

    captures.map(|c| c.get(0).unwrap().as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(init(&"input4_test.txt".to_string()), 13)
    }
}
