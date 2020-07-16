/**
 * Load input data from file as string for given day (01-24).
 */
pub fn load_input(day : String) -> String {
    let path = String::from("./data/input") + &day;
    let input: String = std::fs::read_to_string(path).unwrap();
    return input;
}