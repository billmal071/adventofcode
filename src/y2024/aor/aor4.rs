use std::{io::ErrorKind, num::IntErrorKind};

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1

        // 🎁 Your code here! 🎁
        // handle errors also lol
        let parts = csv_row.split(",").collect::<Vec<_>>();

        if parts.len() != 3 {
          return Err("csv_row has length less than 3")
        }

        // let name = match parts.get(0) {
        //   Some(n) => n.to_string(),
        //   None => return Err("Error: no name"), 
        // }; 
        let mut name = String::new();
        if let Some(n) = parts.get(0) {
          name = n.to_string();
        }

        // let good_deeds = match parts.get(1) {
        //   Some(gd) => gd.parse::<u32>().map_err(|_| "Error")?, 
        //   None => return Err(""), 
        // };
        let mut good_deeds = 0;
        if let Some(gd) = parts.get(1) {
          good_deeds = gd.parse::<u32>().map_err(|_| "error")?
        }

        // let bad_deeds = match parts.get(2) {
        //   Some(bd) => bd.parse::<u32>().map_err(|_| "Error")?,
        //   None => return Err(""),
        // };
        let mut bad_deeds = 0;
        if let Some(bd) = parts.get(2) {
          bad_deeds = bd.parse::<u32>().map_err(|_| "Error")?
        }

        Ok(Self::new(name, good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}
