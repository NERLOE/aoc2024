use regex::Regex;

// Find all instances of the pattern "mul(a,b)" and sum the products of a and b
// Example input:
// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
//
// Example Output: 2*4 + 5*5 + 11*8 + 8*5 = 8 + 25 + 88 + 40 = 161

pub fn main(contents: String) {
    println!("Day 3: Mull It Over");

    // Create a regex pattern to match the "mul(a,b)" pattern and capture the two numbers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Find all instances of the pattern in the input string
    let groups = re.captures_iter(&contents);

    let mut sum = 0;

    // Iterate over the captures and sum the products of the two numbers
    groups.for_each(|cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();

        sum += a * b;
    });

    println!("Sum: {}", sum);
}
