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

pub mod mountain {
    use crate::sim::state::State;
    pub fn update_state(best_state: &mut State, state: &mut State) {
        if best_state.score > state.score {
            best_state.score = state.score.clone();
        } else {
            *state = best_state.clone();
        }
    }
}

