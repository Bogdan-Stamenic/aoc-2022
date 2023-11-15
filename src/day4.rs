pub struct ElfAssignments {
    elf1_lower: u32,
    elf1_upper: u32,
    elf2_lower: u32,
    elf2_upper: u32,
}

impl ElfAssignments {
    fn is_one_superset(&self) -> bool {
        if (self.elf1_lower >= self.elf2_lower) && (self.elf1_upper <= self.elf2_upper) {
            return true; // elf2 superset of elf1
        } else if (self.elf2_lower >= self.elf1_lower) && (self.elf2_upper <= self.elf1_upper) {
            return true; // elf1 superset of elf2
        }
        false
    }

    fn any_overlap(&self) -> bool {
        if (self.elf2_lower <= self.elf1_upper) && (self.elf2_upper >= self.elf1_lower) {
           return true;
        }
        false
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<ElfAssignments> {
    input
        .lines()
        .map(|str| {
            let mut nums = str.split(',')
                .flat_map(|str_pair| str_pair.split('-'))
                .map(|s| s.parse::<u32>().unwrap());
           ElfAssignments {
                elf1_lower: nums.next().unwrap(),
                elf1_upper: nums.next().unwrap(),
                elf2_lower: nums.next().unwrap(),
                elf2_upper: nums.next().unwrap() }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1 (input: &[ElfAssignments]) -> u32 {
    input
        .iter()
        .map(|assignment| if assignment.is_one_superset() {1u32} else {0u32})
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[ElfAssignments]) -> u32 {
    input
        .iter()
        .map(|dat_ass| if dat_ass.any_overlap() {1} else {0})
        .sum()
}
