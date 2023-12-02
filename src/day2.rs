use regex::Regex;
use std::fs;

pub fn init(file_path: &String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let games = contents.split("\n");

    let sum: i32 = games
        .into_iter()
        .map(|game| {
            let id = game_id(game).unwrap();
            let mut splitted_game = game.split(':');
            splitted_game.next();

            match is_possible(splitted_game.next().unwrap().trim()) {
                true => id,
                false => 0,
            }
        })
        .sum();

    println!("{:?}", sum)
}

fn game_id(game: &str) -> Option<i32> {
    let re = Regex::new(r"Game (?<id>\d+):").unwrap();

    let Some(caps) = re.captures(game) else {
        println!("no match!");
        return None;
    };

    match &caps["id"].parse::<i32>() {
        Ok(n) => Some(n.clone()),
        Err(_) => None,
    }
}

fn is_possible(sets: &str) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    sets.split(";").all(|s| {
        let red_result = does_match(s, "red", max_red);
        let green_result = does_match(s, "green", max_green);
        let blue_result = does_match(s, "blue", max_blue);

        return red_result && green_result && blue_result;
    })
}

fn does_match(set: &str, color: &str, max: i32) -> bool {
    let reg = Regex::new(format!(r"(?<number>\d+) {}", color).as_str()).unwrap();

    match reg.captures(set) {
        Some(caps) => &caps["number"].parse::<i32>().unwrap() <= &max,
        None => true,
    }
}
