use std::fs;
/// collection module reads in the file and reads in the included files
pub fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}