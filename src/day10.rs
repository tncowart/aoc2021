fn calc_chunks(line: &str) -> (Vec<char>, bool, Option<char>) {
    let mut stack = Vec::<char>::new();
    let mut valid = true;
    let mut value: Option<char> = None;
    for c in line.chars() {
        if ['{', '(', '[', '<'].contains(&c) {
            stack.push(c);
        } else if ['}', ')', ']', '>'].contains(&c) {
            value = Some(c);
            valid = matches!(
                (stack.pop(), c),
                (Some('['), ']')
                    | (Some('{'), '}')
                    | (Some('<'), '>')
                    | (Some('('), ')')
                    | (None, _)
            );
            if !valid {
                break;
            }
        }
    }
    (stack, valid, value)
}

pub fn day10() {
    let p1: u32 = include_str!("../resources/day10.txt")
        .lines()
        .map(calc_chunks)
        .filter(|s| !s.1)
        .map(|s| match s.2 {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        })
        .sum();

    let mut p2: Vec<u64> = include_str!("../resources/day10.txt")
        .lines()
        .map(calc_chunks)
        .filter(|s| s.1)
        .map(|s| {
            s.0.iter().rev().fold(0, |acc, v| {
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
