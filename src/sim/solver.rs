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

use super::state::State;
pub fn mountain(best_state: &mut State, state: &mut State) {
    //! bese_state(self)を更新する。<br>
    //! 新しいStateのほうが悪いStateの場合は、stateをbest_stateに戻す。
    
    // 最小化の場合は > , 最大化の場合は < 。
    if best_state.score > state.score {
        *best_state = state.clone();
    } else {
        *state = best_state.clone();
    }
}
