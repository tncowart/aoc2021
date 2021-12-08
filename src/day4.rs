const BOARD_WIDTH: usize = 5;
// const BOARD_HEIGHT: usize = 5;

fn calc_value(board: &[Option<i32>], number: i32) -> i32 {
    board.iter().flatten().sum::<i32>() * number
}

fn check_board(board: &[Option<i32>], x: usize, y: usize) -> bool {
    board[(y * BOARD_WIDTH)..((y + 1) * BOARD_WIDTH )]
        .iter()
        .all(|x| *x == None)
        || board
            .iter()
            .skip(x % BOARD_WIDTH)
            .step_by(BOARD_WIDTH)
            .all(|x| *x == None)
}

fn update_board(board: &mut Vec<Option<i32>>, value: i32) -> Option<(usize, usize)> {
    match board.iter().position(|x| *x == Some(value)) {
        Some(pos) => {
            board[pos] = None;
            Some((pos % BOARD_WIDTH, pos / BOARD_WIDTH))
        }
        None => None,
    }
}

fn load_boards(file: std::str::Lines) -> Vec<Vec<Option<i32>>> {
    let mut boards: Vec<Vec<Option<i32>>> = Vec::new();

    let mut board: Vec<Option<i32>> = Vec::new();
    for line in file {
        if line.trim() == "" {
            boards.push(board);
            board = Vec::new();
            continue
        }

        let numbers: Vec<Option<i32>> = line.split_ascii_whitespace().map(|n| Some(n.parse().unwrap())).collect();
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

    let mut p1: i32 = 0;
    'out: for number in numbers {
        for board in boards.iter_mut() {
            if let Some((x, y)) = update_board(board, number) {
                if check_board(board, x, y) {
                    println!("{:?}", board);
                    p1 = calc_value(board, number);
                    break 'out;
                }
            }
        }
    }

    println!("Day 04.1: {}", p1);
    // println!("Day 04.2: {}", life);
}
