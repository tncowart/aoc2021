const BOARD_WIDTH: usize = 5;
// const BOARD_HEIGHT: usize = 5;

fn calc_value(board: &Option<Vec<Option<i32>>>, number: i32) -> i32 {
    board.iter().flatten().flatten().sum::<i32>() * number
}

fn check_board(board: &Option<Vec<Option<i32>>>, x: usize, y: usize) -> bool {
    if let Some(board) = board {
        board[(y * BOARD_WIDTH)..((y + 1) * BOARD_WIDTH)]
            .iter()
            .all(|x| *x == None)
            || board
                .iter()
                .skip(x % BOARD_WIDTH)
                .step_by(BOARD_WIDTH)
                .all(|x| *x == None)
    } else {
        false
    }
}

fn update_board(board: &mut Option<Vec<Option<i32>>>, value: i32) -> Option<(usize, usize)> {
    if let Some(board) = board {
        match board.iter().position(|x| *x == Some(value)) {
            Some(pos) => {
                board[pos] = None;
                Some((pos % BOARD_WIDTH, pos / BOARD_WIDTH))
            }
            None => None,
        }
    } else {
        None
    }
}

fn load_boards(file: std::str::Lines) -> Vec<Option<Vec<Option<i32>>>> {
    let mut boards: Vec<Option<Vec<Option<i32>>>> = Vec::new();

    let mut board: Vec<Option<i32>> = Vec::new();
    for line in file {
        if line.trim() == "" {
            boards.push(Some(board));
            board = Vec::new();
            continue;
        }

        let numbers: Vec<Option<i32>> = line
            .split_ascii_whitespace()
            .map(|n| Some(n.parse().unwrap()))
            .collect();
        board.extend_from_slice(&numbers);
    }

    boards
}

pub fn day4() {
    let mut lines = include_str!("../resources/day4.txt").lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();
    lines.next();

    let mut boards = load_boards(lines);

    let mut p1: i32 = -1;
    let mut p2: i32 = 0;
    for number in numbers.iter() {
        let mut i = 0;
        while i < boards.len() {
            if let Some((x, y)) = update_board(&mut boards[i], *number) {
                if check_board(&boards[i], x, y) {
                    p2 = calc_value(&boards[i], *number);
                    if p1 < 0 {
                        p1 = p2;
                    }
                    boards[i] = None;
                }
            }
            i += 1;
        }
    }

    println!("Day 04.1: {}", p1);
    println!("Day 04.2: {}", p2);
}
