mod utils;

use js_sys::Float64Array;
use wasm_bindgen::prelude::*;

mod dragon;
mod koch;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen]
pub fn generate_koch_curve_pathes(width: f64, height: f64, max_iter: usize) -> Vec<Float64Array> {
    let mut pathes = koch::Points::new(width, height);

    let pathes = pathes.generate(max_iter);
    pathes
        .body
        .into_iter()
        .map(|v| {
            let float_array = Float64Array::new_with_length(2);
            float_array.copy_from(&[v.x, v.y]);

            float_array
        })
        .collect::<Vec<_>>()
}

#[wasm_bindgen]
pub fn generate_dragon_pathes(width: f64, height: f64, max_iter: usize) -> Vec<Float64Array> {
    let mut pathes = dragon::Points::new(width, height);

    let pathes = pathes.generate(max_iter);
    pathes
        .body
        .into_iter()
        .map(|v| {
            let float_array = Float64Array::new_with_length(2);
            float_array.copy_from(&[v.x, v.y]);

            float_array
        })
        .collect::<Vec<_>>()
}
