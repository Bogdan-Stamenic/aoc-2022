use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split('\n').map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            assert!(s1.len() == s2.len()); // sizes don't match -> panic
            let mut set1: HashSet<u8> = s1.bytes().collect::<HashSet<u8>>();
            let set2 = s2.bytes().collect::<HashSet<u8>>();
            set1.retain(|c| set2.contains(c));
            set1.into_iter()
                .map(|n| {
                    n as u32 - b'A' as u32 + 1u32 + if n.is_ascii_uppercase() { 26u32 } else { 0 }
                })
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    input
        .chunks(3)
        .flat_map(|group| {
            group
                .iter()
                .map(|str| str.bytes().collect::<HashSet<_>>())
                .reduce(|acc, e| acc.intersection(&e).cloned().collect())
                .unwrap()
        })
        .map(|x| x as u32 - b'A' as u32 + 1u32 + if x.is_ascii_uppercase() { 26u32 } else { 0 })
        .sum()
}
