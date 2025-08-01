use egui::{Context, Ui};
use crate::mouse::MouseController;
use crate::config::Settings;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AutoMouseApp {
    mouse_controller: Arc<Mutex<MouseController>>,
    settings: Settings,
    is_active: bool,
    last_activity: Instant,
    status_text: String,
}

impl AutoMouseApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui가 한글을 지원하도록 폰트를 설정합니다.
        let mut fonts = egui::FontDefinitions::default();

        // D2Coding 폰트를 추가합니다.
        // 프로젝트 루트에 assets/D2Coding.ttf 파일을 위치시켜야 합니다.
        fonts.font_data.insert(
            "d2coding".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/D2Coding.ttf")),
        );

        // 기본 폰트(Proportional)와 고정폭 폰트(Monospace) 패밀리에
        // D2Coding을 최우선으로 추가합니다.
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "d2coding".to_owned());
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "d2coding".to_owned());

        // egui 컨텍스트에 폰트 설정을 적용합니다.
        cc.egui_ctx.set_fonts(fonts);


        let settings = Settings::load().unwrap_or_default();
        let mouse_controller = Arc::new(Mutex::new(MouseController::new()));
        
        Self {
            mouse_controller,
            settings,
            is_active: false,
            last_activity: Instant::now(),
            status_text: "대기 중".to_string(),
        }
    }
    
    fn update_status(&mut self) {
        if self.is_active {
            let elapsed = self.last_activity.elapsed();
            self.status_text = format!(
                "활성화됨 - 마지막 동작: {}초 전",
                elapsed.as_secs()
            );
        } else {
            self.status_text = "비활성화됨".to_string();
        }
    }
}

impl eframe::App for AutoMouseApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.update_status();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stay Awake...");
            ui.separator();
            
            // 상태 표시
            ui.label(format!("상태: {}", self.status_text));
            ui.separator();
            
            // 설정 섹션
            ui.heading("Settings");
            self.settings_ui(ui);
            
            ui.separator();
            
            // 제어 섹션
            ui.heading("Controls");
            self.control_ui(ui);
            
            ui.separator();
            
            // 통계 섹션
            ui.heading("Statistics");
            self.stats_ui(ui);
        });
        
        // 자동 업데이트
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

impl AutoMouseApp {
    fn settings_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("동작 간격 (초):");
            ui.add(egui::Slider::new(&mut self.settings.interval_seconds, 1.0..=300.0));
        });
        
        ui.horizontal(|ui| {
            ui.label("이동 거리 (픽셀):");
            ui.add(egui::Slider::new(&mut self.settings.move_distance, 1..=100));
        });
        
        ui.checkbox(&mut self.settings.enable_sound, "소리 알림");
        ui.checkbox(&mut self.settings.start_minimized, "시작 시 최소화");
        
        if ui.button("설정 저장").clicked() {
            if let Err(e) = self.settings.save() {
                self.status_text = format!("설정 저장 실패: {}", e);
            } else {
                self.status_text = "설정이 저장되었습니다".to_string();
            }
        }
    }
    
    fn control_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button(if self.is_active { "중지" } else { "시작" }).clicked() {
                self.is_active = !self.is_active;
                self.last_activity = Instant::now();
                
                if self.is_active {
                    if let Ok(mut controller) = self.mouse_controller.lock() {
                        controller.start(self.settings.clone());
                    }
                } else {
                    if let Ok(mut controller) = self.mouse_controller.lock() {
                        controller.stop();
                    }
                }
            }
            
            if ui.button("테스트 동작").clicked() {
                if let Ok(mut controller) = self.mouse_controller.lock() {
                    controller.move_mouse(self.settings.move_distance);
                }
            }
        });
    }
    
    fn stats_ui(&mut self, ui: &mut Ui) {
        if let Ok(controller) = self.mouse_controller.lock() {
            ui.label(format!("총 동작 횟수: {}", controller.get_total_moves()));
            ui.label(format!("마지막 동작 시간: {}", 
                controller.get_last_move_time().format("%H:%M:%S")));
        }
    }
} 