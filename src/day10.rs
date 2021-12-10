pub fn day10() {
    let p1: u32 = include_str!("../resources/day10.txt")
        .lines()
        .map(|line| {
            let mut stack = Vec::<char>::new();
            let mut error = ' ';
            for c in line.chars() {
                if ['{', '(', '[', '<'].contains(&c) {
                    stack.push(c);
                } else if ['}', ')', ']', '>'].contains(&c) {
                    error = match (stack.pop(), c) {
                        (Some('['), ']')
                        | (Some('{'), '}')
                        | (Some('<'), '>')
                        | (Some('('), ')')
                        | (None, _) => ' ',
                        _ => c,
                    };
                    if error != ' ' {
                        break;
                    }
                }
            }
            error
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum();

    let mut p2: Vec<u64> = include_str!("../resources/day10.txt")
        .lines()
        .map(|line| {
            let mut stack = Vec::<char>::new();
            let mut valid = true;
            for c in line.chars() {
                if ['{', '(', '[', '<'].contains(&c) {
                    stack.push(c);
                } else if ['}', ')', ']', '>'].contains(&c) {
                    valid &= matches!(
                        (stack.pop(), c),
                        (Some('['), ']')
                            | (Some('{'), '}')
                            | (Some('<'), '>')
                            | (Some('('), ')')
                            | (None, _)
                    );
                }
            }
            if !valid {
                stack.clear()
            }
            stack
        })
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.iter().rev().fold(0, |acc, v| {
                acc * 5
                    + match v {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            })
        })
        .collect();
    p2.sort_unstable();

    println!("Day 10.1: {}", p1);
    println!("Day 10.2: {}", p2[p2.len() / 2]);
}
