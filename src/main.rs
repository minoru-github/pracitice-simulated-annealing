
mod time;
mod cost_function;
mod input;
mod sim;
use crate::sim::Sim;
mod state;
mod solver;

fn main() {
    let sim = Sim::new();
    sim.run();
}
