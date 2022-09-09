use rand_pcg::Mcg128Xsl64;

use crate::time;
use crate::input::Input;
use crate::sim::Sim;
use crate::state::State;

pub fn mountain() {
    let input = Input::read();

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

    best_state.output();
}
