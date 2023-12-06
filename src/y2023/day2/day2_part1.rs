use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::Index;

pub fn cube_conundrum() {
  let map_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
  let regex = Regex::new(r"(\d+)\s*(red|green|blue)").unwrap();
  let mut plausible_games: Vec<&str> = Vec::new();
  let binding = fs::read_to_string("src/y2023/day2/day2.txt").unwrap();

  // read the file
  for line in binding.lines()
  {
    let split: Vec<_> = regex.captures_iter(line).collect();
    // println!("{:?}", split);
    let game = line.split(':').collect::<Vec<&str>>();
    let number = game[0].split(" ").collect::<Vec<&str>>();
    println!("The number from game {}, original vector {:?}", number[1], game);
    plausible_games.push(number[1]);
    println!("All the games in a vector: {:?}", plausible_games);
    // split second part of game
    let second_part: Vec<_> = regex.captures_iter(game[1]).collect();
    // println!("{:?}", second_part);
    for cap in second_part {
      let value = cap[1].parse::<i32>().unwrap();
      let color = &cap[2];
      // println!("Color: {}, Value: {}", color, value);
      if map_cubes.get(color).unwrap() < &value {
        // println!("Game is not possible: {}", number[1]);
        plausible_games.retain(|&x| x != number[1]);
      }
    }
  }
  // println!("All the games in a vector: {:?} finally", plausible_games);
  // convert to number first
  let plausible_games: Vec<i32> = plausible_games.iter().map(|&x| x.parse::<i32>().unwrap()).collect();
  println!("{:?}", plausible_games.iter().sum::<i32>());
}
