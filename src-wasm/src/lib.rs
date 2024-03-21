mod utils;
pub mod trainer;

use tracing::{info, warn};
use trainer::WAVE_RES;
use wasm_bindgen::prelude::*;

const AGENT_COUNT: usize = 50;

static mut TRAINER: Option<trainer::Trainer<AGENT_COUNT>> = None;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    tracing_wasm::set_as_global_default();

    info!("Initialised")
}

#[wasm_bindgen]
pub fn init_training(goal: &[f32]) {
    unsafe { TRAINER = Some(trainer::Trainer::<AGENT_COUNT>::new_rand(goal)) }

    info!("Initialised training session");
}

#[wasm_bindgen]
pub fn step_training() {
    unsafe {
        if let Some(trainer) = TRAINER.as_mut() {
            trainer.step();
        } else {
            warn!("Trainer must be set up before stepping");
        }
        // info!("Stepped training session: {TRAINER:?}");
    }
}

#[wasm_bindgen]
pub fn get_best_output() -> Vec<f32> {
    unsafe {
        let best = TRAINER.as_ref().unwrap().best();
        (0..WAVE_RES).map(|i| best.evaluate((i as f32 / WAVE_RES as f32) * 2. - 1.)).collect()
    }
}

#[wasm_bindgen]
pub fn get_best_fitness() -> f32 {
    unsafe { TRAINER.as_ref().unwrap().best().fitness }
}

#[wasm_bindgen]
pub fn get_best_formula() -> String {
    unsafe { TRAINER.as_ref().unwrap().best().to_string() }
}