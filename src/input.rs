use proconio::{
    input,
    marker::{Bytes, Chars},
};

use crate::cost_function::CostType;

#[derive(Debug, Clone)]
pub struct Input {
    pub initial_x: CostType,
}

impl Input {
    pub fn read() -> Self {
        // a : 型
        // (a,b) : (型, 型)
        // a_vec : [型;サイズ]
        // a_vec2 : [[型;サイズ];サイズ]
        // S : [char; n] or Chars ← Vec<char>
        // s_vec : [String; n]
        // bytes : Bytes ← Vec<u8>
        input! {
            initial_x:CostType,
        };

        Input { initial_x }
    }

    fn debug(result: &Result<Input, &str>) {
        println!("{:?}", result);
    }
}
