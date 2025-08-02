#![cfg_attr(all(not(debug_assertions), windows), windows_subsystem = "windows")] // Windows 릴리즈 빌드 시 콘솔 창 숨기기

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
            .with_inner_size(egui::vec2(300.0, 200.0)) // 축소 상태 크기로 시작
            .with_min_inner_size(egui::vec2(280.0, 180.0))
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Stay awake",
        options,
        Box::new(|cc| Box::new(AutoMouseApp::new(cc))),
    )
}
