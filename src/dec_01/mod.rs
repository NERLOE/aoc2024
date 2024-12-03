pub fn main(contents: String) {
    println!("Day 1: Historian Hysteria");

    // Split the contents of the file into a list of strings by the newline character
    let list: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    // Create two arrays from the list of strings,
    // by splitting each string by two spaces
    // and parsing the first and second element as i32
    let mut arr_1 = list
        .iter() // iterate over the array
        .map(|line| {
            line.split("   ").collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    let mut arr_2 = list
        .iter()
        .map(|line| {
            line.split("   ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>();

    // Sort the arrays, so we can compare them.
    // We use the unstable sort, because we don't care about the order of equal elements and it's faster.
    arr_1.sort_unstable();
    arr_2.sort_unstable();

    let mut sum = 0;

    // Iterate over the arrays and calculate the sum of the absolute differences
    for i in 0..arr_1.len() {
        let difference = (arr_1[i] - arr_2[i]).abs();
        sum += difference;
    }

    println!("Sum: {}", sum);
}
