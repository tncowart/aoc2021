use std::cmp::{ Ordering, Reverse };
use std::collections::{ BinaryHeap, HashMap };
use std::hash::{Hash, Hasher};


const MAP_WIDTH: usize = 100;
const BIG_MAP_WIDTH: usize = MAP_WIDTH * 5;
const MAP_HEIGHT: usize = 100;
const BIG_MAP_HEIGHT: usize = MAP_HEIGHT * 5;

#[derive(Copy, Clone, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x,
            y,
        }
    }

    fn neighbors(&self, width: usize, height: usize) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if self.x % width != 0 {
            neighbors.push(Point::new(self.x - 1, self.y))
        }
        if self.x % width < height - 1 {
            neighbors.push(Point::new(self.x + 1, self.y))
        }
        if self.y > 0 {
            neighbors.push(Point::new(self.x, self.y - 1))
        }
        if self.y < height - 1 {
            neighbors.push(Point::new(self.x, self.y + 1))
        }

        neighbors
    }

    fn manhattan_dist(&self, other: &Point) -> usize {
        let horiz = self.x.max(other.x) - self.x.min(other.x);
        let vert = self.y.max(other.y) - self.y.min(other.y);
        horiz + vert
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


#[derive(Eq, Debug)]
struct Node {
    loc: Point,
    f_weight: usize,
}

impl Node {
    fn new(x: usize, y: usize, f_weight: usize) -> Self {
        Node {
            loc: Point::new(x, y),
            f_weight
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_weight.cmp(&other.f_weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_weight == other.f_weight
    }
}

fn get(map: &[usize], width: usize, loc: &Point) -> usize {
    map[loc.y * width + loc.x]
}

fn set(map: &mut [usize], width: usize,loc: &Point, value: usize) {
    map[loc.y * width + loc.x] = value;
}

fn a_star(map: &[u8], width: usize, height: usize, start: Point, goal: Point) -> Option<usize> {
    let start_node = Node::new(start.x, start.y, start.manhattan_dist(&goal));

    let mut open_set = BinaryHeap::<Reverse<Node>>::new();

    let mut g_map = vec![usize::MAX; map.len()];
    set(&mut g_map, width, &start, 0);


    let mut f_map = vec![usize::MAX; map.len()];
    set(&mut f_map, width, &start, start_node.f_weight);

    open_set.push(Reverse(start_node));

    let mut came_from = HashMap::<Point, Point>::new();

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;
        if current.loc == goal {
            return Some(get(&g_map, width, &goal))
        }

        for n in current.loc.neighbors(width, height) {
            let tentative_g_score = get(&g_map, width, &current.loc) + map[n.y * width + n.x] as usize;
            if tentative_g_score < get(&g_map, width, &n) {
                came_from.entry(n).and_modify(|e| *e = current.loc).or_insert(current.loc);
                set(&mut g_map, width, &n, tentative_g_score);
                let f_score = tentative_g_score + n.manhattan_dist(&goal);
                set(&mut f_map, width, &n, f_score);
                if !open_set.iter().any(|x| x.0.loc == n) {
                    open_set.push(Reverse(Node::new(n.x, n.y, f_score)));
                }
            }
        }
    }
    None
}

pub fn day15() {
    let map_str = include_str!("../resources/day15.txt").replace("\n", "");
    let map = map_str.as_bytes().iter().map(|c| {
        match c {
            48..=57 => c - 48,
            _ => *c
        }
    }).collect::<Vec<u8>>();
    if let Some(len) = a_star(&map, MAP_WIDTH, MAP_HEIGHT, Point::new(0, 0), Point::new(99, 99)) {
        println!("Day 15.1: {}", len);
    }

    let mut big_map: Vec<u8> = vec![0; BIG_MAP_WIDTH * BIG_MAP_HEIGHT];
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let val = map[y * MAP_WIDTH + x];
            for dx in 0..5 {
                for dy in 0..5 {
                    let mut new_val = (val + (dx as u8) + (dy as u8)) % 10;
                    if new_val < val {
                        new_val += 1;
                    }
                    let new_x = x + (MAP_WIDTH * dx);
                    let new_y = y + (MAP_HEIGHT * dy);
                    big_map[new_y * BIG_MAP_WIDTH + new_x] = new_val;
                }
            }
        }
    }

    if let Some(len) = a_star(&big_map, BIG_MAP_WIDTH, BIG_MAP_HEIGHT, Point::new(0, 0), Point::new(499, 499)) {
        println!("Day 15.2: {}", len);
    }
}