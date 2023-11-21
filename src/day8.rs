#![allow(dead_code)]
pub struct Forest {
    m_columns: usize,
    n_rows: usize,
    arr: Vec<u32>,
}

impl Forest {
    /* row first ("c"-style) ordering */
    fn index2d(&self, x: usize, y: usize) -> u32 {
        self.arr[(x  * self.m_columns) + y]
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        if (x == 0) || (x == self.n_rows - 1) {
            return true;
        }
        if (y == 0) || (y == self.m_columns - 1) {
            return true;
        }
        let tree: u32 = self.index2d(x, y);
        let mut block_count = 0;
        for idx in (0..=(x-1)).rev() {
            if self.index2d(idx, y) >= tree {block_count += 1; break;}
        }
        for idx in (x+1)..self.n_rows {
            if self.index2d(idx, y) >= tree {block_count += 1; break;}
        }
        for idx in (0..=(y-1)).rev() {
            if self.index2d(x, idx) >= tree {block_count += 1; break;}
        }
        for idx in (y+1)..self.m_columns {
            if self.index2d(x, idx) >= tree {block_count += 1; break;}
        }
        if block_count == 4 {
            return false;
        }
        true
    }

    fn calc_scenic_score(&self, x: usize, y: usize) -> u32 {
        /* Trees on edge have scenic score of zero */
        if (x == 0) || (x == self.n_rows - 1) {
            return 0;
        }
        if (y == 0) || (y == self.m_columns - 1) {
            return 0;
        }
        let tree: u32 = self.index2d(x, y);
        let mut left = 0;
        let mut right = 0;
        let mut up = 0;
        let mut down = 0;
        for idx in (0..=(x-1)).rev() {
            left += 1;
            if self.index2d(idx, y) >= tree {break;}
        }
        for idx in (x+1)..self.n_rows {
            right += 1;
            if self.index2d(idx, y) >= tree {break;}
        }
        for idx in (0..=(y-1)).rev() {
            down += 1;
            if self.index2d(x, idx) >= tree {break;}
        }
        for idx in (y+1)..self.m_columns {
            up += 1;
            if self.index2d(x, idx) >= tree {break;}
        }
        left * right * down * up
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Forest {
    let m_clm: usize = input.lines()
        .take(1)
        .map(|str: &str| str.len())
        .sum();
    let n_rws: usize = input.lines().count();
    let mut my_vec = Vec::<u32>::new();
    for line in input.lines() {
        for num in line.chars() {
            my_vec.push(u32::from(num) - (b'1' as u32 - 1u32));
        }
    }
    let out = Forest {
        m_columns: m_clm,
        n_rows: n_rws,
        arr: my_vec,
    };
    out
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Forest) -> u32 {
    /* Use math to calculate edges */
    let mut ans = 2 * input.m_columns + 2 * input.n_rows - 4;
    /* Check all trees on the inside */
    for y_idx in 1..(input.n_rows - 1) {
        for x_idx in 1..(input.m_columns - 1) {
            if input.is_tree_visible(x_idx, y_idx) {ans += 1;}
        }
    }
    ans as u32
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Forest) -> u32 {
    let mut ans = 0;
    for y_idx in 1..(input.n_rows - 1) {
        for x_idx in 1..(input.m_columns - 1) {
            let new_score = input.calc_scenic_score(x_idx, y_idx);
            if new_score > ans {ans = new_score;}
        }
    }
    ans as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn test_day8_generator() {
        let out = input_generator(TEST_INPUT);
        /* Check if sizes okay */
        assert_eq!(out.m_columns, 5 as usize);
        assert_eq!(out.n_rows, 5 as usize);
        assert_eq!(out.arr.len(), 25);
        /* Four corners */
        assert_eq!(out.index2d(0,0), 3);
        assert_eq!(out.index2d(0,4), 3);
        assert_eq!(out.index2d(4,0), 3);
        assert_eq!(out.index2d(4,4), 0);
        /* A few other points */
        assert_eq!(out.index2d(4,3), 9);
        assert_eq!(out.index2d(2,4), 2);
    }
    
    #[test]
    fn test_day8p1() {
        let out = input_generator(TEST_INPUT);
        let ans = solve_part1(&out);
        assert_eq!(ans,21)
    }

    #[test]
    fn test_day8p2() {
        let out = input_generator(TEST_INPUT);
        let ans = solve_part2(&out);
        assert_eq!(ans,8)
    }
}
