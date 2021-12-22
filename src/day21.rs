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

fn dirac2(p1_pos: u64, p1_score: u64, p2_pos: u64, p2_score: u64) -> (u64, u64) {
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
    let (p23, p13) = dirac2(p2_pos, p2_score, p1_pos3, p1_score + p1_pos3);
    let (p24, p14) = dirac2(p2_pos, p2_score, p1_pos4, p1_score + p1_pos4);
    let (p25, p15) = dirac2(p2_pos, p2_score, p1_pos5, p1_score + p1_pos5);
    let (p26, p16) = dirac2(p2_pos, p2_score, p1_pos6, p1_score + p1_pos6);
    let (p27, p17) = dirac2(p2_pos, p2_score, p1_pos7, p1_score + p1_pos7);
    let (p28, p18) = dirac2(p2_pos, p2_score, p1_pos8, p1_score + p1_pos8);
    let (p29, p19) = dirac2(p2_pos, p2_score, p1_pos9, p1_score + p1_pos9);
    (p13 + 3 * p14 + 6 * p15 + 7 * p16 + 6 * p17 + 3 * p18 + p19, p23 + 3 * p24 + 6 * p25 + 7 * p26 + 6 * p27 + 3 * p28 + p29)
}

pub fn day21() {
    let (p1_score, p2_score, roll_ct) = dirac1(6, 3);
    println!("Day 21.1: {}", p1_score.min(p2_score) * roll_ct);

    let (p1_wins, p2_wins) = dirac2(6, 0, 3, 0);
    println!("Day 21.2: {}-{}", p1_wins, p2_wins);
}