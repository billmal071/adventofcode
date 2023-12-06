use regex::Regex;
use std::fs;

pub fn trebuchet() -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"\d+").unwrap();

    for line in fs::read_to_string("src/day1.txt").unwrap().lines() {
        let matches = re
            .find_iter(line)
            .map(|mat| mat.as_str())
            .collect::<Vec<_>>()
            .join("");
        // println!("{:?}", matches);
        let nums = format!(
            "{}{}",
            &matches[0..1],
            &matches[matches.len() - 1..]
        );
        let nums = nums.parse::<i32>().unwrap();
        // println!("{}", nums);
        // strip to get only numbers
        // let num = line
        //     .chars()
        //     .filter(|c| c.is_ascii_digit())
        //     .collect::<String>();
        //
        // let nums = format!("{}{}", &num[0..1], &num[num.len() - 1..]);
        // let nums = nums.parse::<i32>().unwrap();
        sum += nums;
    }
    // println!("{}", sum);
    sum
}
