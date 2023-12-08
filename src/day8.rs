use std::{collections::HashMap, fs};

pub fn init(file_path: &String) -> i32 {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines = content.lines();

    let instructions = lines.next().unwrap();
    // skipping empty line
    lines.next();

    let mut dico: HashMap<String, (String, String)> = HashMap::new();

    lines.into_iter().for_each(|line| {
        let entries: Vec<&str> = line.matches(char::is_alphanumeric).collect();
        let mut chunks = entries.chunks(3).map(|f| f.join(""));

        // https://doc.rust-lang.org/std/primitive.slice.html#method.chunks to regroup in vec
        dico.insert(
            chunks.next().unwrap(),
            (chunks.next().unwrap(), chunks.next().unwrap()),
        );
    });

    let mut current_value = "AAA";
    let mut cpt: i32 = 0;

    while current_value != "ZZZ" {
        for element in instructions.chars().into_iter() {
            let (left, right) = dico.get(current_value).unwrap();
            current_value = match element == 'L' {
                true => left,
                false => right,
            };

            cpt = cpt + 1;
            if current_value == "ZZZ" {
                break;
            }
        }
    }

    cpt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(init(&"input8_test.txt".to_string()), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(init(&"input8-2_test.txt".to_string()), 6)
    }

    #[test]
    fn test3() {
        assert_eq!(init(&"input8.txt".to_string()), 14893)
    }
}
