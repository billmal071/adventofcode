pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty,
}

pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness,
}

// Move yesterday's function to an associated function in the struct
impl Kid {
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> Niceness {
        if good_deeds == 0 && bad_deeds == 0 {
            return Niceness::Naughty;
        }
        let gd_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = gd_deeds / (gd_deeds + bad_deeds);

        if ratio >= 0.75 {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        }
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        // Return a Kid instance
        Kid {
            name,
            niceness: Self::is_nice(good_deeds, bad_deeds),
        }
    }
}