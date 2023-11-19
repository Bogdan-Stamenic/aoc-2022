#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

/* from Lilith */
fn are_all_different_by_key<T, F, R>(arr: &[T], accessor: F) -> bool
    where F: Fn(&T) -> R, R: PartialEq
{
    arr.iter()
        .enumerate()
        .all(|(idx, a)| {
            arr[(idx+1)..].iter()
                .all(|b| accessor(a) != accessor(b))
        })
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let my_vec : Vec<(usize,u8)> = input
        .bytes()
        .enumerate()
        .collect();
    my_vec
        .as_slice()
        .windows(4)
        .filter(|win| {
            are_all_different_by_key(win, |(_,val)| *val)
        })
        .take(1)
        .map(|win| {
            let (start,_) = win[0];
            4 + start as u32
        })
        .sum()
}

/* Mean no filters : 14.54us */
#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let my_window_size = 14;
    let my_vec : Vec<(usize,u8)> = input
        .bytes()
        .enumerate()
        .collect();
    my_vec
        .as_slice()
        .windows(my_window_size)
        .filter(|win| {
            are_all_different_by_key(win, |(_,val)| *val)
        })
        .take(1)
        .map(|win| {
            let (start,_) = win[0];
            my_window_size as u32 + start as u32
        })
        .sum()
}
