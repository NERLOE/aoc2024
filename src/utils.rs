use std::fs;

// Parse the contents of a file into a string
pub fn parse_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect(format!("Couldn't read the file {file_path}").as_str());
}

// Get the input file path for a specific day in the format "dec_01" or "dec_02" etc.
pub fn get_input_file_path(day: &str) -> String {
    return format!("src/{}/input.txt", day);
}

// Get the input for a specific day in the format "dec_01" or "dec_02" etc.
pub fn get_input(day: &str) -> String {
    return parse_file(&get_input_file_path(day));
}
