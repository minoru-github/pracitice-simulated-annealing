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

use crate::sim::cost_function::CostType;
use crate::sim::input::Input;

#[derive(Debug, Clone)]
pub struct State {
    pub x: CostType,
    pub score: CostType,
}

impl State {
    pub fn new(input: &Input) -> Self {
        State {
            x: input.initial_x,
            score: CostType::max_value(),
        }
    }

    pub fn change(&mut self, rng: &mut Mcg128Xsl64) {
        let val = rng.gen_range(-3, 4);
        self.x += val;
    }

    pub fn output(&self) {
        eprintln!("x: {}, score: {}", self.x, self.score);
    }
}
