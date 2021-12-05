fn vec2bits(vec: &[u32]) -> u32 {
    vec.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, item)| acc | item << i)
}

fn most_common_values(lines: &[Vec<u32>]) -> Vec<u32> {
    let mut acc = vec![0; lines[0].len()];
    for pos in (0..12).rev() {
        acc[pos] = most_common_value(lines, pos)
    }
    acc
}

fn most_common_value(lines: &[Vec<u32>], pos: usize) -> u32 {
    let count = ((lines.len() + lines.len() % 2) / 2) as u32;
    let acc = lines.iter().fold(0, |acc, l| acc + l[pos]);
    (acc >= count) as u32
}

fn life_support(lines: &[Vec<u32>], most: u32) -> u32 {
    let mut gases: Vec<Vec<u32>> = lines.to_vec();
    let mut gas = vec![0; lines[0].len()];
    let mut mask = 12;

    while gases.len() != 1 {
        mask -= 1;
        let pos = 11 - mask;
        gas[pos] = (most_common_value(&gases, pos) == most) as u32;

        gases = gases
            .iter()
            .cloned()
            .filter(|l| vec2bits(&gas) >> mask == vec2bits(l) >> mask)
            .collect();
    }

    vec2bits(&gases[0])
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

    let gamma = vec2bits(&most_common_values(&lines));
    let epsilon = !gamma & 0b00000000000000000000111111111111;
    let power = gamma * epsilon;
    assert_eq!(2250414, power);
    println!("Day 03.1: {}", power);

    let o2 = life_support(&lines, 1);
    let co2 = life_support(&lines, 0);
    let life = o2 * co2;
    assert_eq!(6085575, life);
    println!("Day 03.2: {}", life);
}
