fn vec2bits(vec: &[u32]) -> u32 {
    let mut result = 0;

    for (i, item) in vec.iter().rev().enumerate() {
        result |= item << i;
    }

    result
}

fn most_common_values(lines: &[Vec<u32>]) -> Vec<u32> {
    let mut acc = vec![0; lines[0].len()];
    let count = (lines.len() / 2) as u32;

    for line in lines {
        for (i, val) in line.iter().enumerate() {
            acc[i] += val;
        }
    }

    acc.iter()
        .map(|i| if i >= &count { 1 } else { 0 })
        .collect()
}

fn most_common_value(lines: &[Vec<u32>], pos: usize) -> u32 {
    let mut acc = 0;
    let count = (lines.len() / 2) as u32;

    for line in lines {
        acc += line[pos];
    }

    if acc >= count {
        1
    } else {
        0
    }
}

fn least_common_value(lines: &[Vec<u32>], pos: usize) -> u32 {
    let mut acc = 0;
    let mut count = (lines.len() / 2) as u32;
    if lines.len() % 2 == 1 {
        count += 1;
    }

    for line in lines {
        acc += line[pos];
    }

    if acc >= count {
        0
    } else {
        1
    }
}

fn _day3(lines: &[Vec<u32>]) -> (u32, u32) {
    let gamma_vec = most_common_values(lines);
    let gamma = vec2bits(&gamma_vec);

    let epsilon_vec = gamma_vec
        .iter()
        .map(|i| if i == &1 { 0 } else { 1 })
        .collect::<Vec<u32>>();
    let epsilon = vec2bits(&epsilon_vec);

    let power_consumption = gamma * epsilon;

    let mut oxygens: Vec<Vec<u32>> = lines.to_vec();
    let mut oxygen = vec![0; lines[0].len()];
    let mut mask = 12;

    while oxygens.len() != 1 {
        mask -= 1;
        let mut new_oxygens: Vec<Vec<u32>> = Vec::new();
        let pos = 11 - mask;
        oxygen[pos] = most_common_value(&oxygens, pos);
        let most_common = vec2bits(&oxygen);

        for line in oxygens {
            if most_common >> mask == vec2bits(&line) >> mask {
                new_oxygens.push(line.clone());
            }
        }

        oxygens = new_oxygens;
    }

    let mut co2s: Vec<Vec<u32>> = lines.to_vec();
    let mut co2 = vec![0; lines[0].len()];
    mask = 12;

    while co2s.len() != 1 {
        mask -= 1;
        let mut new_c02s: Vec<Vec<u32>> = Vec::new();
        let pos = 11 - mask;
        co2[pos] = least_common_value(&co2s, pos);
        let least_common = vec2bits(&co2);

        for line in co2s {
            if least_common >> mask == vec2bits(&line) >> mask {
                new_c02s.push(line.clone());
            }
        }

        co2s = new_c02s;
    }

    let o2 = vec2bits(&oxygens[0]);
    let co2 = vec2bits(&co2s[0]);

    (power_consumption, o2 * co2)
}

pub fn day3() {
    let lines: Vec<Vec<u32>> = include_str!("../resources/day3.txt")
        .lines()
        .map(|l| {
            l.split("")
                .filter(|i| !i.is_empty())
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let x = _day3(&lines);
    println!("Day 03.1: {}", x.0);
    println!("Day 03.2: {}", x.1);
}
