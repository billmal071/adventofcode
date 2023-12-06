use std::fs;
use regex::Regex;

// find the max number for the colors, then multiply, then add them together
pub fn cube() {
  let regex = Regex::new(r"(\d+)\s*(red|green|blue)").unwrap();
  let binding = fs::read_to_string("src/y2023/day2/day2.txt").unwrap();
  let mut total_vec = Vec::new();

  for line in binding.lines()
  {
    let game = line.split(':').collect::<Vec<&str>>();
    let number = game[0].split(" ").collect::<Vec<&str>>();
    // split second part of game
    let second_part: Vec<_> = regex.captures_iter(game[1]).collect();
    // println!("{:?}", second_part);
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for cap in second_part {
      let value = cap[1].parse::<i32>().unwrap();
      let color = &cap[2];
      // match color {
      //   "red" => {
      //     if max_red < value {
      //       max_red = value;
      //     }
      //   }
      //   "green" => {
      //     if max_green < value {
      //       max_green = value;
      //     }
      //   }
      //   "blue" => {
      //     if max_blue < value {
      //       max_blue = value;
      //     }
      //   }
      //   _ => ()
      // }
      if color.eq("red") {
        if max_red < value {
          max_red = value;
        }
      } else if color.eq("green") {
        if max_green < value {
          max_green = value;
        }
      } else if color.eq("blue") {
        if max_blue < value {
          max_blue = value;
        }
      }
    }
    total_vec.push(max_red * max_blue * max_green);
  }
  println!("Total vec: {:?}", total_vec);
  println!("Sum of vec: {:?}", total_vec.iter().sum::<i32>());
}