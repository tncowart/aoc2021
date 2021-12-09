const BOARD_WIDTH: usize = 1000;
const BOARD_HEIGHT: usize = 1000;

fn parse_line(line: &str) -> Vec<i32> {
    line.split(" -> ").map(|x| x.split(',').map(|y| y.parse::<i32>().unwrap())).flatten().collect()
}

fn draw_line(board: &mut Vec<u32>, line: Vec<i32>) {
    let x_change = (line[2] - line[0]).clamp(-1, 1);
    let y_change = (line[3] - line[1]).clamp(-1, 1);
    let mut x = line[0];
    let mut y = line[1];
    while x != line[2] || y != line[3] {
        board[y as usize * BOARD_WIDTH + x as usize] += 1;
        x += x_change;
        y += y_change;
    }
    board[y as usize * BOARD_WIDTH + x as usize] += 1;
}

pub fn day5() {
    let mut board1: Vec<u32> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];
    let mut board2: Vec<u32> = vec![0; BOARD_WIDTH * BOARD_HEIGHT];

    include_str!("../resources/day5.txt").lines().map(parse_line).filter(|l| l[0] == l[2] || l[1] == l[3]).for_each(|l| draw_line(&mut board1, l));
    include_str!("../resources/day5.txt").lines().map(parse_line).for_each(|l| draw_line(&mut board2,  l));

    println!("Day 05.1: {}", board1.iter().filter(|i| **i > 1).count());
    println!("Day 05.2: {}", board2.iter().filter(|i| **i > 1).count());
}