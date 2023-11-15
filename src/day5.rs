struct InstructionSet {
    num_times : u32,
    from : u32,
    to : u32
}

pub struct CratesAndInstructions {
    crates : Vec<Vec<char>>,
    instructions : Vec<InstructionSet>
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> CratesAndInstructions {
    let mut it = input.split("\n\n");
    let crates_in : &str = it.next().unwrap();
    let instructions_in : &str = it.next().unwrap();
    /* Figure out where the crates_idx are*/
    let dock_nums : &str = crates_in
        .lines().last().unwrap();
    let crates_idx = dock_nums.bytes()
        .enumerate()
        .filter(|(_,c)| *c != b' ')
        .map(|(idx,_)| idx)
        .collect::<Vec<_>>();
    /* Crate starting positions */
    let mut crate_positions = Vec::<Vec<char>>::new();
    for num in crates_idx.iter() {
        let contents : Vec<char> = crates_in
            .lines()
            .rev()
            .skip(1)
            .flat_map(|line : &str| line.bytes().nth(*num as usize))
            .map(|c| c as char)
            .filter(|c| *c != ' ')
            .collect();
        crate_positions.push(contents);
    }
    /* Read into instructions */
    let instructions_list : Vec<InstructionSet> = instructions_in
        .lines()
        .map(|line : &str| {
            let mut nums_from_input = line
                .split_ascii_whitespace()
                .filter(|str| !str.contains("o"))//all keywords have "o"
                .map(|x| x.parse::<u32>().unwrap())
                .map(|x| x - 1); /* make nums fit for vec-indexing */
            InstructionSet {
                num_times : nums_from_input.next().unwrap(),
                from : nums_from_input.next().unwrap(),
                to : nums_from_input.next().unwrap()}
        })
    .collect();
    /* Return parsed puzzle input */
    CratesAndInstructions {
        crates : crate_positions,
        instructions : instructions_list,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1 (input: &CratesAndInstructions) -> String {
    let CratesAndInstructions {crates : crts, instructions : instr} = input;
    let mut crates = crts.clone();
    for x in instr {
        for _ in 0..x.num_times {
            let var = crates[x.from as usize].pop();
            let var = match var {
                Some(t) => t,
                None => break,
            };
            crates[x.to as usize].push(var);
        }
    }
    crts
        .into_iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

//#[aoc(day5, part2)]
//pub fn solve_part2(input: &[ElfAssignments]) -> u32 
