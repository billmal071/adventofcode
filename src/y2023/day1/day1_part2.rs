use regex::{Captures, Regex};
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

pub fn trebuchet() -> i32 {
    let map_of_string_nums = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum = 0;

    // do a replace of the parts of the string that match the ones in the hashmap
    let regex = Regex::new(
        &map_of_string_nums
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>()
            .join("|"),
    )
    .unwrap();
    // println!("{:?}", regex);
    let curr_dir = env::current_dir().unwrap();
    let file_path = curr_dir.join("y2023/day1/day1.txt");
    // let path = Path::new("./day1.txt");
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // strip to get numbers and words matching the key in the hashmap
        let result = regex.replace_all(line, |caps: &Captures| {
            map_of_string_nums
                .get(caps.get(0).unwrap().as_str())
                .unwrap()
                .to_string()
        });
        println!("{} - {}", result, line);

        let num = result
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        let nums = format!("{}{}", &num[0..1], &num[num.len() - 1..]);
        let nums = nums.parse::<i32>().unwrap();
        sum += nums;
    }
    println!("{}", sum);
    sum
}
