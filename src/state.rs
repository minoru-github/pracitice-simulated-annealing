use crate::cost_function::CostType;
use crate::input::Input;

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

    pub fn output(&self) {
        eprintln!("x: {}, score: {}", self.x, self.score);
    }
}