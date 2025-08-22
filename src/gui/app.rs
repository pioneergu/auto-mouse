use crate::config::Settings;
use crate::mouse::MouseController;
use crate::timer::SimpleTimer;
use egui::{Context, Ui};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AutoMouseApp {
    mouse_controller: Arc<Mutex<MouseController>>,
    settings: Settings,
    is_active: bool,
    last_activity: Instant,
    status_text: String,
    is_collapsed: bool,
    last_collapsed_state: bool,

    // 타이머 추가
    timer: SimpleTimer,
    
    // 최소화 처리를 위한 플래그
    should_minimize: bool,
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
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "d2coding".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "d2coding".to_owned());

        // egui 컨텍스트에 폰트 설정을 적용합니다.
        cc.egui_ctx.set_fonts(fonts);

        let settings = Settings::load().unwrap_or_default();
        let mouse_controller = Arc::new(Mutex::new(MouseController::new()));
        let timer = SimpleTimer::new(settings.timer_minutes);

        Self {
            mouse_controller,
            should_minimize: settings.start_minimized,
            settings,
            is_active: false,
            last_activity: Instant::now(),
            status_text: "대기 중".to_string(),
            is_collapsed: true,
            last_collapsed_state: true,
            timer,
        }
    }

    fn update_status(&mut self) {
        // 타이머 만료 체크
        if self.settings.enable_timer && self.timer.is_expired() && self.is_active {
            self.is_active = false;
            if let Ok(mut controller) = self.mouse_controller.lock() {
                controller.stop();
            }
            self.status_text = "타이머 만료로 자동 중지됨".to_string();
            return;
        }

        if self.is_active {
            let elapsed = self.last_activity.elapsed();
            let mut status = format!("활성화됨 - 마지막 동작: {}초 전", elapsed.as_secs());

            // 타이머 정보 추가
            if self.settings.enable_timer {
                let remaining = self.timer.get_remaining_seconds();
                let minutes = remaining / 60;
                let seconds = remaining % 60;
                status.push_str(&format!(" ({}분 {}초 남음)", minutes, seconds));
            }

            self.status_text = status;
        } else {
            self.status_text = "비활성화됨".to_string();
        }
    }
}

impl eframe::App for AutoMouseApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 첫 번째 업데이트에서 시작 시 최소화 처리
        if self.should_minimize {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Minimized(true));
            self.should_minimize = false;
        }

        self.update_status();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stay Awake...");
            ui.separator();

            // 상태 표시
            ui.label(format!("상태: {}", self.status_text));
            ui.separator();

            self.collapsible_ui(ui, ctx);

            // 창 크기를 콘텐츠에 맞게 자동 조정
            ui.allocate_space(ui.available_size());
        });

        // 축소/확대 상태가 변경될 때 창 크기 자동 조정
        if self.should_resize() {
            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::Resizable(true));

            // 상태에 따른 적절한 창 크기 설정
            let target_size = if self.is_collapsed {
                egui::vec2(300.0, 200.0) // 축소 시 크기
            } else {
                egui::vec2(300.0, 450.0) // 확대 시 크기
            };

            ctx.send_viewport_cmd(egui::viewport::ViewportCommand::InnerSize(target_size));
            self.last_collapsed_state = self.is_collapsed;
        }

        // 자동 업데이트
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

impl AutoMouseApp {
    fn collapsible_ui(&mut self, ui: &mut Ui, _ctx: &Context) {
        // Controls는 항상 표시 (맨 위로 이동)
        ui.heading("Controls");
        self.control_ui(ui);

        ui.separator();

        // 확대/축소 버튼
        let button_text = if self.is_collapsed {
            "▼ 확대"
        } else {
            "▲ 축소"
        };
        if ui.button(button_text).clicked() {
            self.is_collapsed = !self.is_collapsed;
        }

        if !self.is_collapsed {
            // Expanded view: show other sections
            ui.separator();

            ui.heading("Settings");
            self.settings_ui(ui);

            ui.separator();

            ui.heading("Statistics");
            self.stats_ui(ui);
        }

        // 창 크기를 콘텐츠에 맞게 자동 조정
        ui.ctx().request_repaint();
    }

    fn settings_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("동작 간격 (초):");
            ui.add(egui::Slider::new(
                &mut self.settings.interval_seconds,
                1.0..=300.0,
            ));
        });

        ui.horizontal(|ui| {
            ui.label("이동 거리 (픽셀):");
            ui.add(egui::Slider::new(&mut self.settings.move_distance, 1..=100));
        });

        ui.checkbox(&mut self.settings.enable_sound, "소리 알림");
        ui.checkbox(&mut self.settings.start_minimized, "시작 시 최소화");

        ui.separator();

        // 타이머 설정 UI
        ui.checkbox(&mut self.settings.enable_timer, "자동 타이머 사용");

        if self.settings.enable_timer {
            ui.horizontal(|ui| {
                ui.label("타이머 시간 (분):");
                if ui
                    .add(egui::Slider::new(&mut self.settings.timer_minutes, 1..=480).text("분"))
                    .changed()
                {
                    self.timer.set_duration(self.settings.timer_minutes);
                }
            });

            // 타이머 상태 표시
            match self.timer.get_state() {
                crate::timer::simple_timer::TimerState::Stopped => {
                    ui.label("타이머: 비활성화");
                }
                crate::timer::simple_timer::TimerState::Running => {
                    let remaining = self.timer.get_remaining_seconds();
                    let minutes = remaining / 60;
                    let seconds = remaining % 60;
                    ui.label(format!("남은 시간: {}분 {}초", minutes, seconds));
                }
                crate::timer::simple_timer::TimerState::Expired => {
                    ui.colored_label(egui::Color32::RED, "타이머: 시간 만료");
                }
            }
        }

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
            if ui
                .button(if self.is_active { "중지" } else { "시작" })
                .clicked()
            {
                self.is_active = !self.is_active;
                self.last_activity = Instant::now();

                if self.is_active {
                    // 마우스 컨트롤러 시작
                    if let Ok(mut controller) = self.mouse_controller.lock() {
                        controller.start(self.settings.clone());
                    }

                    // 타이머 시작 (설정이 활성화된 경우)
                    if self.settings.enable_timer {
                        self.timer.start();
                    }
                } else {
                    // 마우스 컨트롤러 중지
                    if let Ok(mut controller) = self.mouse_controller.lock() {
                        controller.stop();
                    }

                    // 타이머 중지
                    self.timer.stop();
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
            ui.label(format!(
                "마지막 동작 시간: {}",
                controller.get_last_move_time().format("%H:%M:%S")
            ));
        }
    }

    fn should_resize(&self) -> bool {
        self.is_collapsed != self.last_collapsed_state
    }
}
