#![allow(unused)]

use aoc_runner_derive::aoc;
use itertools::izip;

#[cfg(test)]
const S: usize = 15;

#[cfg(not(test))]
const S: usize = 141;

unsafe fn load_inp(inp: &[u8]) -> ([u16; S * S], Vec<(isize, isize, isize)>) {
    let mut map = [0u16; S * S];
    let mut start = 0usize;
    let mut end = 0usize;
    let mut path: Vec<(isize, isize, isize)> = Vec::with_capacity(9500);
    for r in 0..S {
        for c in 0..S {
            match *inp.get_unchecked(r * (S + 1) + c) {
                b'#' => *map.get_unchecked_mut(r * S + c) = u16::MAX,
                b'S' => start = r * S + c,
                b'E' => end = r * S + c,
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
    let mut cur = start;
    let mut len = 0;
    while cur != end {
        path.push(((cur / S) as isize, (cur % S) as isize, cur as isize));
        *map.get_unchecked_mut(cur) = len;
        len += 1;

        if *map.get_unchecked(cur - S) == 0 {
            cur -= S
        } else if *map.get_unchecked(cur + 1) == 0 {
            cur += 1
        } else if *map.get_unchecked(cur + S) == 0 {
            cur += S
        } else {
            cur -= 1
        }
    }
    *map.get_unchecked_mut(end) = len;
    path.push(((end / S) as isize, (end % S) as isize, end as isize));
    (map, path)
}

#[cfg(test)]
const B: u16 = 20;

#[cfg(not(test))]
const B: u16 = 100;

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let (map, _) = load_inp(inp);
    // Vertical walls
    // let mut saves = [0; 65];
    let mut cnt = 0;
    for (&l, &m, &r) in izip!(map.iter(), map[1..].iter(), map[2..].iter()) {
        if m == u16::MAX && l != u16::MAX && r != u16::MAX {
            // wall with on 2 sides empty space
            if (l.abs_diff(r) - 2) >= B {
                cnt += 1
            }
            // let dif = ;
            // saves[(l.abs_diff(r) - 2) as usize] += 1;
        }
    }
    // Horizontal walls
    for (&t, &m, &b) in izip!(map.iter(), map[S..].iter(), map[2 * S..].iter()) {
        if m == u16::MAX && t != u16::MAX && b != u16::MAX {
            // wall with on 2 sides empty space
            if (t.abs_diff(b) - 2) >= B {
                cnt += 1
            }
        }
    }
    cnt
}

const RM: [(isize, isize, u16); 420] = [(-1, 0, 1), (0, 1, 1), (-2, 0, 2), (-1, 1, 2), (0, 2, 2), (1, 1, 2), (-3, 0, 3), (-2, 1, 3), (-1, 2, 3), (0, 3, 3), (1, 2, 3), (2, 1, 3), (-4, 0, 4), (-3, 1, 4), (-2, 2, 4), (-1, 3, 4), (0, 4, 4), (1, 3, 4), (2, 2, 4), (3, 1, 4), (-5, 0, 5), (-4, 1, 5), (-3, 2, 5), (-2, 3, 5), (-1, 4, 5), (0, 5, 5), (1, 4, 5), (2, 3, 5), (3, 2, 5), (4, 1, 5), (-6, 0, 6), (-5, 1, 6), (-4, 2, 6), (-3, 3, 6), (-2, 4, 6), (-1, 5, 6), (0, 6, 6), (1, 5, 6), (2, 4, 6), (3, 3, 6), (4, 2, 6), (5, 1, 6), (-7, 0, 7), (-6, 1, 7), (-5, 2, 7), (-4, 3, 7), (-3, 4, 7), (-2, 5, 7), (-1, 6, 7), (0, 7, 7), (1, 6, 7), (2, 5, 7), (3, 4, 7), (4, 3, 7), (5, 2, 7), (6, 1, 7), (-8, 0, 8), (-7, 1, 8), (-6, 2, 8), (-5, 3, 8), (-4, 4, 8), (-3, 5, 8), (-2, 6, 8), (-1, 7, 8), (0, 8, 8), (1, 7, 8), (2, 6, 8), (3, 5, 8), (4, 4, 8), (5, 3, 8), (6, 2, 8), (7, 1, 8), (-9, 0, 9), (-8, 1, 9), (-7, 2, 9), (-6, 3, 9), (-5, 4, 9), (-4, 5, 9), (-3, 6, 9), (-2, 7, 9), (-1, 8, 9), (0, 9, 9), (1, 8, 9), (2, 7, 9), (3, 6, 9), (4, 5, 9), (5, 4, 9), (6, 3, 9), (7, 2, 9), (8, 1, 9), (-10, 0, 10), (-9, 1, 10), (-8, 2, 10), (-7, 3, 10), (-6, 4, 10), (-5, 5, 10), (-4, 6, 10), (-3, 7, 10), (-2, 8, 10), (-1, 9, 10), (0, 10, 10), (1, 9, 10), (2, 8, 10), (3, 7, 10), (4, 6, 10), (5, 5, 10), (6, 4, 10), (7, 3, 10), (8, 2, 10), (9, 1, 10), (-11, 0, 11), (-10, 1, 11), (-9, 2, 11), (-8, 3, 11), (-7, 4, 11), (-6, 5, 11), (-5, 6, 11), (-4, 7, 11), (-3, 8, 11), (-2, 9, 11), (-1, 10, 11), (0, 11, 11), (1, 10, 11), (2, 9, 11), (3, 8, 11), (4, 7, 11), (5, 6, 11), (6, 5, 11), (7, 4, 11), (8, 3, 11), (9, 2, 11), (10, 1, 11), (-12, 0, 12), (-11, 1, 12), (-10, 2, 12), (-9, 3, 12), (-8, 4, 12), (-7, 5, 12), (-6, 6, 12), (-5, 7, 12), (-4, 8, 12), (-3, 9, 12), (-2, 10, 12), (-1, 11, 12), (0, 12, 12), (1, 11, 12), (2, 10, 12), (3, 9, 12), (4, 8, 12), (5, 7, 12), (6, 6, 12), (7, 5, 12), (8, 4, 12), (9, 3, 12), (10, 2, 12), (11, 1, 12), (-13, 0, 13), (-12, 1, 13), (-11, 2, 13), (-10, 3, 13), (-9, 4, 13), (-8, 5, 13), (-7, 6, 13), (-6, 7, 13), (-5, 8, 13), (-4, 9, 13), (-3, 10, 13), (-2, 11, 13), (-1, 12, 13), (0, 13, 13), (1, 12, 13), (2, 11, 13), (3, 10, 13), (4, 9, 13), (5, 8, 13), (6, 7, 13), (7, 6, 13), (8, 5, 13), (9, 4, 13), (10, 3, 13), (11, 2, 13), (12, 1, 13), (-14, 0, 14), (-13, 1, 14), (-12, 2, 14), (-11, 3, 14), (-10, 4, 14), (-9, 5, 14), (-8, 6, 14), (-7, 7, 14), (-6, 8, 14), (-5, 9, 14), (-4, 10, 14), (-3, 11, 14), (-2, 12, 14), (-1, 13, 14), (0, 14, 14), (1, 13, 14), (2, 12, 14), (3, 11, 14), (4, 10, 14), (5, 9, 14), (6, 8, 14), (7, 7, 14), (8, 6, 14), (9, 5, 14), (10, 4, 14), (11, 3, 14), (12, 2, 14), (13, 1, 14), (-15, 0, 15), (-14, 1, 15), (-13, 2, 15), (-12, 3, 15), (-11, 4, 15), (-10, 5, 15), (-9, 6, 15), (-8, 7, 15), (-7, 8, 15), (-6, 9, 15), (-5, 10, 15), (-4, 11, 15), (-3, 12, 15), (-2, 13, 15), (-1, 14, 15), (0, 15, 15), (1, 14, 15), (2, 13, 15), (3, 12, 15), (4, 11, 15), (5, 10, 15), (6, 9, 15), (7, 8, 15), (8, 7, 15), (9, 6, 15), (10, 5, 15), (11, 4, 15), (12, 3, 15), (13, 2, 15), (14, 1, 15), (-16, 0, 16), (-15, 1, 16), (-14, 2, 16), (-13, 3, 16), (-12, 4, 16), (-11, 5, 16), (-10, 6, 16), (-9, 7, 16), (-8, 8, 16), (-7, 9, 16), (-6, 10, 16), (-5, 11, 16), (-4, 12, 16), (-3, 13, 16), (-2, 14, 16), (-1, 15, 16), (0, 16, 16), (1, 15, 16), (2, 14, 16), (3, 13, 16), (4, 12, 16), (5, 11, 16), (6, 10, 16), (7, 9, 16), (8, 8, 16), (9, 7, 16), (10, 6, 16), (11, 5, 16), (12, 4, 16), (13, 3, 16), (14, 2, 16), (15, 1, 16), (-17, 0, 17), (-16, 1, 17), (-15, 2, 17), (-14, 3, 17), (-13, 4, 17), (-12, 5, 17), (-11, 6, 17), (-10, 7, 17), (-9, 8, 17), (-8, 9, 17), (-7, 10, 17), (-6, 11, 17), (-5, 12, 17), (-4, 13, 17), (-3, 14, 17), (-2, 15, 17), (-1, 16, 17), (0, 17, 17), (1, 16, 17), (2, 15, 17), (3, 14, 17), (4, 13, 17), (5, 12, 17), (6, 11, 17), (7, 10, 17), (8, 9, 17), (9, 8, 17), (10, 7, 17), (11, 6, 17), (12, 5, 17), (13, 4, 17), (14, 3, 17), (15, 2, 17), (16, 1, 17), (-18, 0, 18), (-17, 1, 18), (-16, 2, 18), (-15, 3, 18), (-14, 4, 18), (-13, 5, 18), (-12, 6, 18), (-11, 7, 18), (-10, 8, 18), (-9, 9, 18), (-8, 10, 18), (-7, 11, 18), (-6, 12, 18), (-5, 13, 18), (-4, 14, 18), (-3, 15, 18), (-2, 16, 18), (-1, 17, 18), (0, 18, 18), (1, 17, 18), (2, 16, 18), (3, 15, 18), (4, 14, 18), (5, 13, 18), (6, 12, 18), (7, 11, 18), (8, 10, 18), (9, 9, 18), (10, 8, 18), (11, 7, 18), (12, 6, 18), (13, 5, 18), (14, 4, 18), (15, 3, 18), (16, 2, 18), (17, 1, 18), (-19, 0, 19), (-18, 1, 19), (-17, 2, 19), (-16, 3, 19), (-15, 4, 19), (-14, 5, 19), (-13, 6, 19), (-12, 7, 19), (-11, 8, 19), (-10, 9, 19), (-9, 10, 19), (-8, 11, 19), (-7, 12, 19), (-6, 13, 19), (-5, 14, 19), (-4, 15, 19), (-3, 16, 19), (-2, 17, 19), (-1, 18, 19), (0, 19, 19), (1, 18, 19), (2, 17, 19), (3, 16, 19), (4, 15, 19), (5, 14, 19), (6, 13, 19), (7, 12, 19), (8, 11, 19), (9, 10, 19), (10, 9, 19), (11, 8, 19), (12, 7, 19), (13, 6, 19), (14, 5, 19), (15, 4, 19), (16, 3, 19), (17, 2, 19), (18, 1, 19), (-20, 0, 20), (-19, 1, 20), (-18, 2, 20), (-17, 3, 20), (-16, 4, 20), (-15, 5, 20), (-14, 6, 20), (-13, 7, 20), (-12, 8, 20), (-11, 9, 20), (-10, 10, 20), (-9, 11, 20), (-8, 12, 20), (-7, 13, 20), (-6, 14, 20), (-5, 15, 20), (-4, 16, 20), (-3, 17, 20), (-2, 18, 20), (-1, 19, 20), (0, 20, 20), (1, 19, 20), (2, 18, 20), (3, 17, 20), (4, 16, 20), (5, 15, 20), (6, 14, 20), (7, 13, 20), (8, 12, 20), (9, 11, 20), (10, 10, 20), (11, 9, 20), (12, 8, 20), (13, 7, 20), (14, 6, 20), (15, 5, 20), (16, 4, 20), (17, 3, 20), (18, 2, 20), (19, 1, 20)];

#[cfg(test)]
const SI: isize = 15;
#[cfg(not(test))]
const SI: isize = 141;

unsafe fn part2_inner(inp: &[u8]) -> u32 {
    let (map, path) = load_inp(inp);
    let map_ptr = map.as_ptr();
    // let mut cnts = [0; 100];
    let mut cnt = 0;
    for (row, col, s) in path {
        for (ro, co, off) in RM {
            if let (0..SI, 0..SI) = (row + ro, col + co) {
                let t = map_ptr.offset(s + ro * SI + co).read();
                if t != u16::MAX && t.abs_diff(map_ptr.offset(s).read()) > off + B - 1 {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

#[aoc(day20, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day20, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let inp = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
        assert_eq!(part1(inp), 5);
        assert_eq!(part2(inp), 1232)
    }
}
