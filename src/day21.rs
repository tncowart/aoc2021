use std::collections::HashMap;

const BOARD_SIZE: u32 = 10;
const P1_GOAL: u32 = 1000;
const P2_GOAL: u64 = 21;

fn dirac1(p1_start: u32, p2_start: u32) -> (u32, u32, u32) {
    let mut p1_score = 0;
    let mut p1_pos = p1_start;
    let mut p2_score = 0;
    let mut p2_pos = p2_start;
    let mut die = 2;
    let mut roll_ct = 3;

    loop {
        p1_pos = ((p1_pos - 1 + die * 3) % BOARD_SIZE) + 1;
        p1_score += p1_pos;
        if p1_score >= P1_GOAL {
            break
        }
        die = ((die - 1 + 3) % 100) + 1;
        roll_ct += 3;

        p2_pos = ((p2_pos - 1 + die * 3) % BOARD_SIZE) + 1;
        p2_score += p2_pos;
        if p2_score >= P1_GOAL {
            break
        }
        die = ((die - 1 + 3) % 100) + 1;
        roll_ct += 3;
    }

    (p1_score, p2_score, roll_ct)
}

fn dirac2(p1_pos: u64, p1_score: u64, p2_pos: u64, p2_score: u64, cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>) -> (u64, u64) {
    if let Some(result) = cache.get(&(p1_pos, p1_score, p2_pos, p2_score)) {
        return *result
    }

    if p2_score >= P2_GOAL {
        return (0, 1)
    }

    let p1_pos3 = (((p1_pos - 1) + 3) % BOARD_SIZE as u64) + 1;
    let p1_pos4 = (((p1_pos - 1) + 4) % BOARD_SIZE as u64) + 1;
    let p1_pos5 = (((p1_pos - 1) + 5) % BOARD_SIZE as u64) + 1;
    let p1_pos6 = (((p1_pos - 1) + 6) % BOARD_SIZE as u64) + 1;
    let p1_pos7 = (((p1_pos - 1) + 7) % BOARD_SIZE as u64) + 1;
    let p1_pos8 = (((p1_pos - 1) + 8) % BOARD_SIZE as u64) + 1;
    let p1_pos9 = (((p1_pos - 1) + 9) % BOARD_SIZE as u64) + 1;
    let (p23, p13) = dirac2(p2_pos, p2_score, p1_pos3, p1_score + p1_pos3, cache);
    let (p24, p14) = dirac2(p2_pos, p2_score, p1_pos4, p1_score + p1_pos4, cache);
    let (p25, p15) = dirac2(p2_pos, p2_score, p1_pos5, p1_score + p1_pos5, cache);
    let (p26, p16) = dirac2(p2_pos, p2_score, p1_pos6, p1_score + p1_pos6, cache);
    let (p27, p17) = dirac2(p2_pos, p2_score, p1_pos7, p1_score + p1_pos7, cache);
    let (p28, p18) = dirac2(p2_pos, p2_score, p1_pos8, p1_score + p1_pos8, cache);
    let (p29, p19) = dirac2(p2_pos, p2_score, p1_pos9, p1_score + p1_pos9, cache);
    let result = (p13 + 3 * p14 + 6 * p15 + 7 * p16 + 6 * p17 + 3 * p18 + p19, p23 + 3 * p24 + 6 * p25 + 7 * p26 + 6 * p27 + 3 * p28 + p29);
    cache.entry((p1_pos, p1_score, p2_pos, p2_score)).or_insert(result);
    result
}

pub fn day21() {
    let (p1_score, p2_score, roll_ct) = dirac1(6, 3);
    println!("Day 21.1: {}", p1_score.min(p2_score) * roll_ct);

    let mut cache = HashMap::<(u64, u64, u64, u64), (u64, u64)>::new();
    let (p1_wins, p2_wins) = dirac2(6, 0, 3, 0, &mut cache);
    println!("Day 21.2: {}-{}", p1_wins, p2_wins);
}