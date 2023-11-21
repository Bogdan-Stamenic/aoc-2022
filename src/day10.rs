use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Addx(i32),
    Noop,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            match line.bytes().next().unwrap() {
                b'a' => Op::Addx(
                    line[5..].parse::<i32>().expect("expected integer")
                    ),
                b'n' => Op::Noop,
                _ => panic!("check input file for correct formatting"),
            }
        })
    .collect::<Vec<Op>>()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Op]) -> i32 {
    let mut addition_buffer = VecDeque::<i32>::from([0,0]);
    let mut reg_x = 1;
    let mut cycle_num = 1;
    let mut ans = 0;
    for operation in input.iter() {
        match operation {
            Op::Addx(num) => {//takes two cycles
                reg_x += addition_buffer.pop_front().unwrap();
                addition_buffer.push_back(*num as i32);
                for _ in 0..2 {
                    reg_x += addition_buffer.pop_front().unwrap();
                    addition_buffer.push_back(0);
                    cycle_num += 1;
                    if (cycle_num >= 20) && ((cycle_num - 20) % 40 == 0) {
                        ans += cycle_num * reg_x;
                    }
                }
            },
            Op::Noop => {
                reg_x += addition_buffer.pop_front().unwrap();
                addition_buffer.push_back(0);
                cycle_num += 1;
                if (cycle_num >= 20) && ((cycle_num - 20) % 40 == 0) {
                    ans += cycle_num * reg_x;
                }
            },
        }
    }
    ans
}

//#[aoc(day10, part2)]
//pub fn solve_part2(input: &[Op]) -> u32 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "noop
addx 3
addx -5";

    #[test]
    fn test_day10_generator() {
        let input = input_generator(TEST_INPUT1);
        assert_eq!(input[0], Op::Noop);
        assert_eq!(input[1], Op::Addx(3));
        assert_eq!(input[2], Op::Addx(-5));
    }

    const TEST_INPUT2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";   
    #[test]
    fn test_day10p1() {
        let input = input_generator(TEST_INPUT2);
        let ans = solve_part1(&input);
        assert_eq!(ans, 13140);
    }

    //#[test]
    //fn test_day10p2() {
    //}
}
