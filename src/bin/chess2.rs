#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chess::{setup_fonts}; 
use eframe::NativeOptions;

fn main() {
    let app = chess::core::GameApp::new(); 
    let native_options = NativeOptions::default(); 
    eframe::run_native("中国象棋", native_options, Box::new(|ctx | {
        setup_fonts(&ctx.egui_ctx);
        Box::new(app)
    }) ); 
}

