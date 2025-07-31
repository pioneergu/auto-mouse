mod gui;
mod mouse;
mod config;

use eframe::NativeOptions;
use gui::AutoMouseApp;

fn main() -> Result<(), eframe::Error> {
    // 로깅 초기화
    env_logger::init();
    
    // GUI 애플리케이션 실행
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Auto Mouse - 자리비움 방지",
        options,
        Box::new(|cc| Box::new(AutoMouseApp::new(cc))),
    )
}
