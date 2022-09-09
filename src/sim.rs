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

use crate::cost_function::{ConvexFunction, CostFunction};
use crate::input::Input;
use crate::solver::mountain;
use crate::state::State;
use crate::time;

#[derive(Debug, Clone)]
pub struct Sim {
    cost_function: ConvexFunction,
}

impl Sim {
    pub fn new() -> Self {
        Sim {
            cost_function: ConvexFunction::new(1, 20, 0),
        }
    }

    fn change_state(&self, state: &mut State, rng: &mut Mcg128Xsl64) {
        let val = rng.gen_range(-3, 4);
        state.x += val;
    }

    fn compute_score(&self, state: &mut State) {
        state.score = self.cost_function.f(state.x).unwrap();
    }

    pub fn run(&self) {
        let input = Input::read();

        let sim = Sim::new();

        let mut rng: Mcg128Xsl64 = rand_pcg::Pcg64Mcg::new(890482);

        let mut state = State::new(&input);
        let mut best_state = state.clone();
        while time::update() < 0.3 {
            best_state = state.clone();

            // 近傍探索
            sim.change_state(&mut state, &mut rng);

            // スコア計算
            sim.compute_score(&mut state);

            Self::debug(&best_state, &state);

            mountain::update_state(&mut best_state, &mut state);
        }

        best_state.output();
    }

    fn debug(best_state: &State, state: &State) {
        eprintln!(
            "x : {}, score {}, best_x:{}, best_score:{}",
            state.x, state.score, best_state.x, best_state.score
        );
    }
}
