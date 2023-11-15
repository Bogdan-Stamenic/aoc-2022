pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
    X,
    Y,
    Z,
}

fn input_to_enum(input: Option<char>) -> RockPaperScissors {
    match input.unwrap() {
        'A' => RockPaperScissors::Rock,
        'B' => RockPaperScissors::Paper,
        'C' => RockPaperScissors::Scissors,
        'X' => RockPaperScissors::X,
        'Y' => RockPaperScissors::Y,
        'Z' => RockPaperScissors::Z,
        _ => unreachable!(),
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(RockPaperScissors, RockPaperScissors)> {
    input
        .split('\n')
        .map(|s| {
            (
                input_to_enum(s.chars().nth(0)),
                input_to_enum(s.chars().nth(2)),
            )
        })
        .collect::<Vec<(RockPaperScissors, RockPaperScissors)>>()
}

fn do_i_win(elf: &RockPaperScissors, me: &RockPaperScissors) -> u32 {
    match (elf, me) {
        (RockPaperScissors::Rock, RockPaperScissors::Y) => 6,
        (RockPaperScissors::Paper, RockPaperScissors::Z) => 6,
        (RockPaperScissors::Scissors, RockPaperScissors::X) => 6,
        (RockPaperScissors::Rock, RockPaperScissors::X) => 3,
        (RockPaperScissors::Paper, RockPaperScissors::Y) => 3,
        (RockPaperScissors::Scissors, RockPaperScissors::Z) => 3,
        _ => 0,
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(RockPaperScissors, RockPaperScissors)]) -> u32 {
    input
        .iter()
        .map(|tup| {
            let thingy = match tup.1 {
                RockPaperScissors::X => 1, //rock
                RockPaperScissors::Y => 2, //paper
                RockPaperScissors::Z => 3, //scissors
                _ => panic!(),
            };
            thingy + do_i_win(&tup.0, &tup.1)
        })
        .sum()
}

fn which_to_pick(elf: &RockPaperScissors, me: &RockPaperScissors) -> u32 {
    match (elf, me) {
        /* lose */
        (RockPaperScissors::Rock, RockPaperScissors::X) => 3,
        (RockPaperScissors::Paper, RockPaperScissors::X) => 1,
        (RockPaperScissors::Scissors, RockPaperScissors::X) => 2,
        /* draw */
        (RockPaperScissors::Rock, RockPaperScissors::Y) => 1,
        (RockPaperScissors::Paper, RockPaperScissors::Y) => 2,
        (RockPaperScissors::Scissors, RockPaperScissors::Y) => 3,
        /* win */
        (RockPaperScissors::Rock, RockPaperScissors::Z) => 2,
        (RockPaperScissors::Paper, RockPaperScissors::Z) => 3,
        (RockPaperScissors::Scissors, RockPaperScissors::Z) => 1,
        /* Something unexpected happened */
        _ => panic!(),
    }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(RockPaperScissors, RockPaperScissors)]) -> u32 {
    input
        .iter()
        .map(|tup| {
            let thingy = match tup.1 {
                RockPaperScissors::X => 0, //lose
                RockPaperScissors::Y => 3, //draw
                RockPaperScissors::Z => 6, //win
                _ => panic!(),
            };
            thingy + which_to_pick(&tup.0, &tup.1)
        })
        .sum()
}
