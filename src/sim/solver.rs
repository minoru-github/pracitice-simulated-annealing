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
use super::time;

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

const T0: f64 = 2e3;
//const T1: f64 = 6e2; // 終端温度が高いと最後まで悪いスコアを許容する
const T1: f64 = 6e1; // 終端温度が高いと最後まで悪いスコアを許容する
pub fn sa(best_state: &mut State, state: &mut State, current_time: f64, rng: &mut Mcg128Xsl64) {
    //! 焼きなまし法
    //! https://scrapbox.io/minyorupgc/%E7%84%BC%E3%81%8D%E3%81%AA%E3%81%BE%E3%81%97%E6%B3%95

    static mut T: f64 = T0;
    static mut CNT: usize = 0;
    let temperature = unsafe {
        CNT += 1;
        if CNT % 100 == 0 {
            let t = current_time / time::LIMIT;
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        T
    };

    // 最大化
    let delta = state.score - best_state.score;
    // 最小化の場合は符号反転させる
    let delta = -1 * delta;

    let prob = f64::exp(delta as f64 / temperature).min(1.0);
    eprintln!("best_x {}, x {}, delta {}, T {:?} , CNT {}, prob {:?}", best_state.x,state.x,  delta, temperature, unsafe{CNT}, prob);

    if delta >= 0 {
        eprintln!("best update");
        *best_state = state.clone();
    } else if rng.gen_bool(prob) {
        eprintln!("prob update");
        *best_state = state.clone();
    } else {
        eprintln!("not update");
        *state = best_state.clone();
    }
}
