use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

use crate::cost_function::{ConvexFunction, CostFunction};
use crate::state::State;

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

    pub fn walk(&self, state: &mut State, rng: &mut Mcg128Xsl64) {
        let val = rng.gen_range(-3, 4);
        state.x += val;
    }

    pub fn compute_score(&self, state: &mut State) {
        state.score = self.cost_function.f(state.x).unwrap();
        //eprintln!("x : {}, score {} ", state.x, state.score);
    }
}
