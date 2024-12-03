mod dec_01;
mod dec_02;
mod utils;

const RUN_DAY: &str = "dec_02";

fn main() {
    let start = std::time::Instant::now();

    // Get the contents of the input file as a string
    let contents = utils::get_input(RUN_DAY);

    // Run the main function of the day, based on the RUN_DAY constant
    match RUN_DAY {
        "dec_01" => dec_01::main(contents),
        "dec_02" => dec_02::main(contents),
        _ => println!("No matching day found"),
    }

    println!("Time taken: {}µs", start.elapsed().as_micros());
}
