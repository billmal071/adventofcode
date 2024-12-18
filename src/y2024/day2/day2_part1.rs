use std::fs;

pub fn safe_list() -> u32 {
    let mut data = Vec::new();
    let safe: Vec<bool> = Vec::new();
    let binding = fs::read_to_string("src/y2024/day2/day2.txt").expect("Failed to read file");

    for line in binding.lines() {
      data.push(line.split_whitespace().collect::<Vec<_>>());
    }
    println!("{:?}", data);

    

    2
}
