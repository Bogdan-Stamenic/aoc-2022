use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords2d {
    x: i32,
    y: i32,
}

impl Coords2d {
    fn manhattan_dist(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[allow(dead_code)]
struct Rope {
    head: Coords2d,
    tail: Coords2d,
}

impl Rope {
    /* Expects to move head only 1 Manhattan dist unit at a time */
    fn rope_sim_step(&mut self, movement: &Coords2d) {
        self.move_head(*movement);
        let diff = self.head_tail_diff();
        /* Manhatten distances illustrated:
         * 43234
         * 32123
         * 21 12
         * 32123
         * 43234
         * */
        match diff.manhattan_dist() {
            0 => {/*noop*/},
            1 => {/*noop*/},
            2 => {
                match (diff.x, diff.y) {
                    (_,0) => {self.tail.x += diff.x.signum();},
                    (0,_) => {self.tail.y += diff.y.signum();},
                    (_,_) => {/*noop*/},
                }
            }
            _ => {/* 3 or 4 */
                self.tail.x += diff.x.signum();
                self.tail.y += diff.y.signum();
            },
        }
    }

    fn move_head(&mut self, movement: Coords2d) {
        self.head.x += movement.x;
        self.head.y += movement.y;
    }

    fn head_tail_diff(&self) -> Coords2d {
        Coords2d {
            x: self.head.x - self.tail.x,
            y: self.head.y - self.tail.y,
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Coords2d> {
    let mut out = Vec::<Coords2d>::new();
    for line in input.lines() {
        let num = line[2..].parse::<i32>().unwrap();
        match line.bytes().next().unwrap() {
            b'L' => {
                for _ in 0..num {
                    out.push(Coords2d {x: -1, y: 0})
                }
            },
            b'R' => {
                for _ in 0..num {
                    out.push(Coords2d {x: 1, y: 0})
                }
            },
            b'U' => {
                for _ in 0..num {
                    out.push(Coords2d {x: 0, y: 1})
                }
            },
            b'D' => {
                for _ in 0..num {
                    out.push(Coords2d {x: 0, y: -1})
                }
            },
            _ => unreachable!(),
        }
    }
    out
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Coords2d]) -> u32 {
    let mut my_rope = Rope {
        head: Coords2d { x: 0, y: 0},
        tail: Coords2d { x: 0, y: 0},
    };
    let mut uniq_tail_coords = HashSet::<Coords2d>::new();
    uniq_tail_coords.insert(Coords2d {x: 0, y: 0});
    for step in input {
        my_rope.rope_sim_step(step);
        uniq_tail_coords.insert(my_rope.tail);
    }
    uniq_tail_coords.iter().count() as u32
}

//#[aoc(day9, part2)]
//pub fn solve_part2(input: &[Coords2d]) -> u32 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn test_day9_generator() {
        let ans = input_generator(TEST_INPUT);
        /* R 4 */
        assert_eq!(ans[0].x, 1);
        assert_eq!(ans[1].x, 1);
        assert_eq!(ans[2].x, 1);
        assert_eq!(ans[3].x, 1);
        /* U 4 */
        assert_eq!(ans[4].y, 1);
        assert_eq!(ans[5].y, 1);
        assert_eq!(ans[6].y, 1);
        assert_eq!(ans[7].y, 1);
        /* L 3 */
        assert_eq!(ans[8].x, -1);
        assert_eq!(ans[9].x, -1);
        assert_eq!(ans[10].x, -1);
    }
    
    #[test]
    fn test_day9p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 13);
    }

    //#[test]
    //fn test_day9p2() {
    //}
}
