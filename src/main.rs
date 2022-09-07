#![allow(unused)]
use itertools::Itertools;
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
        let update_state = |best_state: &mut State, state: &mut State| {
            if best_state.score > state.score {
                best_state.score = state.score.clone();
            } else {
                *state = best_state.clone();
            }
        };

        let debug = |best_state: &State, state: &State| {
            eprintln!(
                "x : {}, score {}, best_x:{}, best_score:{}",
                state.x, state.score, best_state.x, best_state.score
            );
        };

        let sim = Sim::new(&input);

        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let mut state = State::new(&input);
        let mut best_state = state.clone();
        while time::update() < 0.3 {
            best_state = state.clone();

            // 近傍探索
            sim.walk(&mut state, &mut rng);

            // スコア計算
            sim.compute_score(&mut state);

            debug(&best_state, &state);

            update_state(&mut best_state, &mut state);
        }

        state
    }
}

#[derive(Debug, Clone)]
pub struct State {
    x: CostType,
    score: CostType,
}

impl State {
    fn new(input: &Input) -> Self {
        State {
            x: input.initial_x,
            score: CostType::max_value(),
        }
    }

    fn output(&self) {
        eprintln!("x: {}, score: {}", self.x, self.score);
    }
}

#[derive(Debug, Clone)]
pub struct Sim {
    cost_function: ConvexFunction,
}

impl Sim {
    fn new(input: &Input) -> Self {
        Sim {
            cost_function: ConvexFunction::new(1, 20, 0),
        }
    }

    fn walk(&self, state: &mut State, rng: &mut Mcg128Xsl64) {
        let val = rng.gen_range(-3, 4);
        state.x += val;
    }

    fn compute_score(&self, state: &mut State) {
        state.score = self.cost_function.f(state.x).unwrap();
        //eprintln!("x : {}, score {} ", state.x, state.score);
    }
}

type CostType = i64;
pub trait CostFunction {
    type Type;
    fn f(self, x: Self::Type) -> Option<Self::Type>;
}

#[derive(Debug, Clone, Copy)]
pub struct ConvexFunction {
    // y = a * (x - p)^2 + q
    a: CostType,
    p: CostType,
    q: CostType,
}

impl ConvexFunction {
    fn new(a: CostType, p: CostType, q: CostType) -> Self {
        ConvexFunction { a, p, q }
    }
}

impl CostFunction for ConvexFunction {
    type Type = CostType;
    fn f(self, x: Self::Type) -> Option<Self::Type> {
        Some(self.a * (x - self.p) * (x - self.p) + self.q)
    }
}

mod my_lib {
    use super::*;
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
    initial_x: CostType,
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
            initial_x:CostType,
        };

        Input { initial_x }
    }

    fn debug(result: &Result<Input, &str>) {
        println!("{:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConvexFunction, CostFunction};

    #[test]
    fn convex_test() {
        let cf = ConvexFunction::new(1, 20, 0);
        assert_eq!(cf.f(0).unwrap(), 400);
        assert_eq!(cf.f(20).unwrap(), 0);
        assert_eq!(cf.f(30).unwrap(), 100);
    }
}
