/**
 * Day 1, part 1
 *
 * Calculate fuel required to launch a given module is based on its mass.
 *
 * Specifically, to find the fuel required for a module, take its mass, divide by three, round down,
 * and subtract 2.
 *
 * For example: For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
 *
 * What is the sum of the fuel requirements for all of the modules on your spacecraft?
 *
 */
pub fn solve(input : String) {
    let result_part1 = solve_part1(input);
    println!("Day 1 Part 1 result is {}", result_part1);
    assert_eq!(result_part1, 3423279);
}

/**
 * Given a module (a line of our input01 in string format), convert to i32 and calculate the fuel
 * requirement before returning the result as i32.
 */
fn fuel_requirement_for_module(module_mass: &str) -> i32 {
    let mass: i32 = match module_mass.parse() {
        Ok(mass) => mass,
        Err(e) => panic!(e),
    };

    return mass / 3 - 2;
}

/**
 * Loop through lines on input01 data and gather the total requirement from sum of each.
 */
fn solve_part1(input : String) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        let requirement = fuel_requirement_for_module(line);
        //println!("{}", requirement);
        total = total + requirement;
    }

    return total;
}