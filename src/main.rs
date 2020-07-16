mod helpers;
mod day01;
mod day02;

/**
 * Boot the beast up.
 */
fn main() {
    day01::solve(helpers::load_input(String::from("01")));
    day02::solve(helpers::load_input(String::from("02")));
}