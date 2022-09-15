#![allow(unused)]
use itertools::Itertools;
use num::{integer::Roots, Integer, ToPrimitive};

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

mod cost_function;
mod input;
mod solver;
mod state;
mod time;

use cost_function::{ConvexFunction, CostFunction, QuarticFunction};
use input::Input;
use state::State;

#[derive(Debug, Clone)]
pub struct Sim {
    input: Input,
    //cost_function: ConvexFunction,
    cost_function: QuarticFunction,
}

impl Sim {
    pub fn new() -> Self {
        let input = Input::read();

        Sim {
            input,
            //cost_function: ConvexFunction::new(1, 0, 10),
            cost_function: QuarticFunction::new(3, -4, -180, 0, 0),
        }
    }

    fn compute_score(&self, state: &mut State) {
        state.score = self.cost_function.f(state.x).unwrap();
    }

    pub fn run(&self) {
        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let mut state = State::new(&self.input);
        self.compute_score(&mut state);
        
        let mut best_state = state.clone();
        loop {
            let current_time = time::update();
            if current_time >= time::LIMIT {
                break;
            }

            // 近傍探索
            state.change(&mut rng);

            // スコア計算
            self.compute_score(&mut state);

            //self.debug(&best_state, &state);

            // 状態更新
            //solver::mountain(&mut best_state, &mut state);
            solver::sa(&mut best_state, &mut state, current_time, &mut rng)
        }

        best_state.output();
    }

    fn debug(&self, best_state: &State, state: &State) {
        eprintln!(
            "x : {}, score {}, best_x:{}, best_score:{}",
            state.x, state.score, best_state.x, best_state.score
        );
    }
}
