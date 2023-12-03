use std::fs;

pub fn trebuchet() -> i32 {
    let mut vec_of_nums: Vec<String> = Vec::new();

    for line in fs::read_to_string("src/day1.txt").unwrap().lines() {
        println!("{}", line);
        // strip to get only numbers
        vec_of_nums.push(
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>(),
        )
    }

    // println!("{:?}", vec_of_nums);
    println!("length of vector is: {}", vec_of_nums.len());

    let mut sum = 0;
    for num in vec_of_nums.iter() {
        // println!("num is: {}", num);
        // pick the first numbers
        let nums = format!("{}{}", &num[0..1], &num[num.len() - 1..]);
        // println!("{}", nums);
        // convert to i32
        let nums = nums.parse::<i32>().unwrap();
        // add to sum
        sum += nums;
    }
    println!("sum is: {}", sum);
    sum
}
