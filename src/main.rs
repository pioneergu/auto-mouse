#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 릴리즈 빌드 시 콘솔 창 숨기기

mod gui;
mod mouse;
mod config;

use eframe::egui;
use gui::AutoMouseApp;

fn main() -> Result<(), eframe::Error> {
    // 로깅 초기화
    env_logger::init();
    
    // GUI 애플리케이션 실행
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(300.0, 300.0))
            .with_min_inner_size(egui::vec2(250.0, 250.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Stay awake",
        options,
        Box::new(|cc| Box::new(AutoMouseApp::new(cc))),
    )
}
