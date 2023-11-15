#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut my_vec = vec![0; input.len()];
    my_vec.clone_from_slice(input);
    my_vec.sort();
    my_vec.iter().rev().take(3).sum()
}
