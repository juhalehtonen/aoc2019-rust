pub fn solve(input : String) {
    let result_part1 = solve_part1(input);
    println!("Part 1 result is {}", result_part1);
}

fn solve_part1(input : String) -> i32 {
    // 1. Turn string into a vector of vectors with 4 elements in each
    let commands: Vec<&str> = input.split_terminator(',').collect();
    println!("{:?}", commands);
    // 2. For each sub vector, treat them as instructions
    return 0;
}