// Each line is a report. Every number is a level. A report is safe when the following conditions are met:
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
//
// Example of reports:
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9

pub fn main(contents: String) {
    println!("Day 2: Red-Nosed Reports");

    // Split the contents of the file into a list of strings by the newline character
    let list: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let safe_reports = list
        .iter()
        .filter(|line| {
            let levels = line
                .split(" ")
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            // Check if the levels are sorted by checking if the levels are either all increasing or all decreasing
            // To make sure all the levels are sorted in the same way, we check if the levels are sorted in ascending order or descending order
            let is_sorted =
                levels.windows(2).all(|w| w[0] <= w[1]) || levels.windows(2).all(|w| w[0] >= w[1]);

            if !is_sorted {
                return false;
            }

            // Check if the levels are safe by checking if the difference between each level is at least 1 and at most 3
            // We use the windows(2) method to get a sliding window of 2 elements
            let is_safe = levels.windows(2).all(|w| {
                let diff = (w[0] - w[1]).abs();

                // Check if the difference is at least 1 and at most 3
                return diff >= 1 && diff <= 3;
            });

            if !is_safe {
                return false;
            }

            return true;
        })
        .count();

    println!("Amount of Safe reports: {}", safe_reports);
}
