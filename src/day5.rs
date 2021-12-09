const BOARD_WIDTH: usize = 1000;
const BOARD_HEIGHT: usize = 1000;

fn parse_line(line: &str) -> Vec<usize> {
    line.split(" -> ").map(|x| x.split(',').map(|y| y.parse::<usize>().unwrap())).flatten().collect()
}

fn update_board(board: &mut Vec<u32>, x: usize, y: usize) {
    board[y * BOARD_WIDTH + x] += 1;
}

pub fn day5() {
    let lines = include_str!("../resources/day5.txt").lines();
    let mut board: Vec<u32> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];

    lines.map(parse_line).filter(|l| l[0] == l[2] || l[1] == l[3]).for_each(|l| {
        println!("{:?}", l);
        let x1 = std::cmp::min(l[0], l[2]);
        let x2 = std::cmp::max(l[0], l[2]);
        let y1 = std::cmp::min(l[1], l[3]);
        let y2 = std::cmp::max(l[1], l[3]);

        for x in x1..=x2 {
            for y in y1..=y2 {
                update_board(&mut board, x, y);
            }
        }
    });

    let p1 = board.iter().filter(|i| **i > 1).count();
    println!("Day 05.1: {}", p1);
    // println!("Day 05.2: {}", p2);
}