use std::collections::HashSet;

const BOARD_WIDTH: usize = 10;

fn spread_flashes(board: &mut Vec<u32>) {
    let mut flashed = HashSet::<usize>::new();
    let mut flashed_count = u8::MAX;
    while flashed_count != 0 {
        flashed_count = 0;
        for pos in 0..board.len() {
            if board[pos] < 10 || flashed.contains(&pos) {
                continue
            }
            flashed_count += 1;
            flashed.insert(pos);

            let north = pos >= BOARD_WIDTH;
            let east = pos % BOARD_WIDTH != (BOARD_WIDTH - 1);
            let west = pos % BOARD_WIDTH != 0;
            let south = pos < board.len() - BOARD_WIDTH;

            if east {
                board[pos + 1] += 1;
            }
            if west {
                board[pos - 1] += 1;
            }

            if north {
                board[pos - BOARD_WIDTH] += 1;
                if east {
                    board[pos - BOARD_WIDTH + 1] += 1;
                }
                if west {
                    board[pos - BOARD_WIDTH - 1] += 1;
                }
            };

            if south {
                board[pos + BOARD_WIDTH] += 1;
                if east {
                    board[pos + BOARD_WIDTH + 1] += 1;
                }
                if west {
                    board[pos + BOARD_WIDTH - 1] += 1;
                }
            };
        }
    }

    for pos in flashed {
        board[pos] = 0;
    }
}

pub fn day11() {
    let mut board: Vec<u32> = include_str!("../resources/day11.txt").chars().map(|c| c.to_digit(10)).flatten().collect();

    let p1: usize = (0..100).map(|_i| {
        (0..board.len()).for_each(|i| board[i] += 1); // increase energy by one
        spread_flashes(&mut board);
        board.iter().filter(|i| **i == 0).count()
    }).sum();
    println!("Day 11.1: {}", p1);

    board = include_str!("../resources/day11.txt").chars().map(|c| c.to_digit(10)).flatten().collect();
    let mut step = 0;
    while board.iter().filter(|i| **i == 0).count() != board.len() {
        step += 1;
        (0..board.len()).for_each(|i| board[i] += 1); // increase energy by one
        spread_flashes(&mut board);
    }
    println!("Day 11.2: {}", step);
}