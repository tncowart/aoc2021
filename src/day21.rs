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

fn dirac2(p1_pos: u64, p1_score: u64, p2_pos: u64, p2_score: u64, turn: u64, universes: u64) -> (u64, u64) {
    if p1_score >= P2_GOAL {
        return (universes, 0)
    }

    if p2_score >= P2_GOAL {
        return (0, universes)
    }

    if turn == 0 {
        let p1_pos3 = (((p1_pos - 1) + 3) % BOARD_SIZE as u64) + 1;
        let p1_pos4 = (((p1_pos - 1) + 4) % BOARD_SIZE as u64) + 1;
        let p1_pos5 = (((p1_pos - 1) + 5) % BOARD_SIZE as u64) + 1;
        let p1_pos6 = (((p1_pos - 1) + 6) % BOARD_SIZE as u64) + 1;
        let p1_pos7 = (((p1_pos - 1) + 7) % BOARD_SIZE as u64) + 1;
        let p1_pos8 = (((p1_pos - 1) + 8) % BOARD_SIZE as u64) + 1;
        let p1_pos9 = (((p1_pos - 1) + 9) % BOARD_SIZE as u64) + 1;
        let (p13, p23) = dirac2(p1_pos3, p1_score + p1_pos3, p2_pos, p2_score, 1, universes);
        let (p14, p24) = dirac2(p1_pos4, p1_score + p1_pos4, p2_pos, p2_score, 1, universes * 3);
        let (p15, p25) = dirac2(p1_pos5, p1_score + p1_pos5, p2_pos, p2_score, 1, universes * 6);
        let (p16, p26) = dirac2(p1_pos6, p1_score + p1_pos6, p2_pos, p2_score, 1, universes * 7);
        let (p17, p27) = dirac2(p1_pos7, p1_score + p1_pos7, p2_pos, p2_score, 1, universes * 6);
        let (p18, p28) = dirac2(p1_pos8, p1_score + p1_pos8, p2_pos, p2_score, 1, universes * 3);
        let (p19, p29) = dirac2(p1_pos9, p1_score + p1_pos9, p2_pos, p2_score, 1, universes);
        (p13 + p14 + p15 + p16 + p17 + p18 + p19, p23 + p24 + p25 + p26 + p27 + p28 + p29)
    } else {
        let p2_pos3 = (((p2_pos - 1) + 3) % BOARD_SIZE as u64) + 1;
        let p2_pos4 = (((p2_pos - 1) + 4) % BOARD_SIZE as u64) + 1;
        let p2_pos5 = (((p2_pos - 1) + 5) % BOARD_SIZE as u64) + 1;
        let p2_pos6 = (((p2_pos - 1) + 6) % BOARD_SIZE as u64) + 1;
        let p2_pos7 = (((p2_pos - 1) + 7) % BOARD_SIZE as u64) + 1;
        let p2_pos8 = (((p2_pos - 1) + 8) % BOARD_SIZE as u64) + 1;
        let p2_pos9 = (((p2_pos - 1) + 9) % BOARD_SIZE as u64) + 1;
        let (p13, p23) = dirac2(p1_pos, p1_score, p2_pos3, p2_score + p2_pos3, 0, universes);
        let (p14, p24) = dirac2(p1_pos, p1_score, p2_pos4, p2_score + p2_pos4, 0, universes * 3);
        let (p15, p25) = dirac2(p1_pos, p1_score, p2_pos5, p2_score + p2_pos5, 0, universes * 6);
        let (p16, p26) = dirac2(p1_pos, p1_score, p2_pos6, p2_score + p2_pos6, 0, universes * 7);
        let (p17, p27) = dirac2(p1_pos, p1_score, p2_pos7, p2_score + p2_pos7, 0, universes * 6);
        let (p18, p28) = dirac2(p1_pos, p1_score, p2_pos8, p2_score + p2_pos8, 0, universes * 3);
        let (p19, p29) = dirac2(p1_pos, p1_score, p2_pos9, p2_score + p2_pos9, 0, universes);
        (p13 + p14 + p15 + p16 + p17 + p18 + p19, p23 + p24 + p25 + p26 + p27 + p28 + p29)
    }
}

pub fn day21() {
    let (p1_score, p2_score, roll_ct) = dirac1(6, 3);
    println!("Day 21.1: {}", p1_score.min(p2_score) * roll_ct);

    let (p1_wins, p2_wins) = dirac2(6, 0, 3, 0, 0, 1);
    println!("Day 21.2: {}-{}", p1_wins, p2_wins);
}