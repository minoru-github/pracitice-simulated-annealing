#![allow(unused)]
use itertools::Itertools;
use my_lib::XY;
use num::{integer::Roots, Integer, ToPrimitive};
use proconio::{
    input,
    marker::{Bytes, Chars},
};
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;
use std::{
    clone,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    iter::FromIterator,
    ops::Range,
    ops::*,
    slice::SliceIndex,
};
use superslice::Ext;

fn main() {
    let input = Input::read();

    let best_state = solver::mountain(&input);

    best_state.output();
}

mod solver {
    use super::*;

    pub fn mountain(input: &Input) -> State {
        let update_state = |best_score: &mut usize, state: &mut State, old_state: &State| {
            if *best_score > state.score {
                *best_score = state.score.clone();
            } else {
                *state = (*old_state).clone();
            }
        };

        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let mut best_score = 0;
        let mut state = State::new(&input);
        while time::update() < 0.3 {
            let old_state = state.clone();

            // 近傍探索

            // スコア計算

            update_state(&mut best_score, &mut state, &old_state);
        }

        state
    }
}

mod time {
    pub(super) fn update() -> f64 {
        static mut STARTING_TIME_MS: Option<f64> = None;
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let time_ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
        unsafe {
            let now = match STARTING_TIME_MS {
                Some(starting_time_ms) => time_ms - starting_time_ms,
                None => {
                    STARTING_TIME_MS = Some(time_ms);
                    0.0 as f64
                }
            };
            now
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    n: usize,
}

impl Input {
    fn read() -> Self {
        // a : 型
        // (a,b) : (型, 型)
        // a_vec : [型;サイズ]
        // a_vec2 : [[型;サイズ];サイズ]
        // S : [char; n] or Chars ← Vec<char>
        // s_vec : [String; n]
        // bytes : Bytes ← Vec<u8>
        input! {
            n:usize,
        };

        Input { n }
    }

    fn debug(result: &Result<Input, &str>) {
        println!("{:?}", result);
    }
}

#[derive(Debug, Clone)]
pub struct State {
    score: usize,
}

impl State {
    fn new(input: &Input) -> Self {
        State { score: 0 }
    }

    fn output(&self) {
        eprintln!("{}", self.score);
    }
}

#[derive(Debug, Clone)]
pub struct Sim {
    n: usize,
}

impl Sim {
    fn new(input: &Input) -> Self {
        Sim { n: input.n }
    }
}

mod my_lib {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    pub struct XY {
        y: usize, // ↓
        x: usize, // →
        width: usize,
    }

    impl XY {
        pub fn new(x: usize, y: usize, width: usize) -> Self {
            XY { x, y, width }
        }

        pub fn to_1d(&self) -> usize {
            self.y * self.width + self.x
        }

        pub fn to_2d(index: usize, width: usize) -> Self {
            XY {
                x: index % width,
                y: index / width,
                width,
            }
        }
    }

    impl Add for XY {
        type Output = Result<XY, &'static str>;
        fn add(self, rhs: Self) -> Self::Output {
            let (x, y) = if cfg!(debug_assertions) {
                // debugではオーバーフローでpanic発生するため、オーバーフローの溢れを明確に無視する(※1.60場合。それ以外は不明)
                (self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
            } else {
                (self.x + rhs.x, self.y + rhs.y)
            };

            if x >= self.width || y >= self.width {
                Err("out of range")
            } else {
                Ok(XY {
                    x,
                    y,
                    width: self.width,
                })
            }
        }
    }
}
