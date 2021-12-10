use std::collections::VecDeque;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn pos_n(x: usize, y: usize) -> usize {
    (y - 1) * WIDTH + x
}

fn pos_e(x: usize, y: usize) -> usize {
    y * WIDTH + x + 1
}

fn pos_s(x: usize, y: usize) -> usize {
    (y + 1) * WIDTH + x
}

fn pos_w(x: usize, y: usize) -> usize {
    y * WIDTH + x - 1
}

fn risk_level(map: &[u32], pos: usize) -> u32 {
    let val = map[pos];
    let (x, y) = (pos % WIDTH, pos / WIDTH);
    let n = y > 0 && map[pos_n(x, y)] > val;
    let e = x < WIDTH - 1 && map[pos_e(x, y)] > val;
    let s = y < HEIGHT - 1 && map[pos_s(x, y)] > val;
    let w = x > 0 && map[pos_w(x, y)] > val;

    if n && e && s && w {
        val + 1
    } else {
        0
    }
}

fn flood_fill(map: &mut [u32], pos: usize) -> u32 {
    if map[pos] == 1 || map[pos] == 9 {
        return 0;
    }

    let mut fill_size = 0;
    let mut to_check: VecDeque<usize> = VecDeque::from([pos]);
    while !to_check.is_empty() {
        let pos = to_check.pop_front().unwrap();
        if map[pos] != 0 {
            continue;
        }
        let (x, y) = (pos % WIDTH, pos / WIDTH);

        map[pos] = 1;
        fill_size += 1;
        if y > 0 && map[pos_n(x, y)] == 0 {
            to_check.push_back(pos_n(x, y))
        }
        if x < WIDTH - 1 && map[pos_e(x, y)] == 0 {
            to_check.push_back(pos_e(x, y))
        }
        if y < HEIGHT - 1 && map[pos_s(x, y)] == 0 {
            to_check.push_back(pos_s(x, y))
        }
        if x > 0 && map[pos_w(x, y)] == 0 {
            to_check.push_back(pos_w(x, y))
        }
    }

    fill_size
}

pub fn day9() {
    let basin_map: Vec<u32> = include_str!("../resources/day9.txt")
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    println!(
        "Day 9.1: {}",
        (0..basin_map.len())
            .map(|i| risk_level(&basin_map, i))
            .sum::<u32>()
    );
    let mut basin_map = basin_map
        .iter()
        .map(|v| if *v != 9 { 0 } else { *v })
        .collect::<Vec<u32>>();
    let mut basin_sizes = (0..basin_map.len())
        .map(|i| flood_fill(&mut basin_map, i))
        .collect::<Vec<u32>>();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    println!("Day 9.2: {}", basin_sizes.iter().take(3).product::<u32>());
}
