// Import the wasm_bindgen prelude module for WebAssembly interaction.
use wasm_bindgen::prelude::*;


// Annotation to indicate that this function is accessible from JavaScript.
#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    if income <= 11600.0 {
        income * 0.10
    } else if income <= 47150.0 {
        1160.0 + (income - 11600.0) * 0.12
    } else if income <= 100525.0 {
        5426.0 + (income - 47150.0) * 0.22
    } else if income <= 191950.0 {
        17168.50 + (income - 100525.0) * 0.24
    } else if income <= 243725.0 {
        39110.50 + (income - 191950.0) * 0.32
    } else if income <= 609350.0 {
        55678.50 + (income - 243725.0) * 0.35
    } else {
        183647.25 + (income - 609350.0) * 0.37
    }
}
