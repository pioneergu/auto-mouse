use crate::config::Settings;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct MouseController {
    is_running: Arc<AtomicBool>,
    total_moves: u64,
    last_move_time: DateTime<Utc>,
}

impl MouseController {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            total_moves: 0,
            last_move_time: Utc::now(),
        }
    }

    pub fn start(&mut self, settings: Settings) {
        if self.is_running.load(Ordering::Relaxed) {
            return;
        }

        self.is_running.store(true, Ordering::Relaxed);
        let is_running = Arc::clone(&self.is_running);

        thread::spawn(move || {
            Self::mouse_worker(is_running, settings);
        });
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }

    pub fn move_mouse(&mut self, distance: i32) {
        if let Err(e) = self.perform_mouse_move(distance) {
            eprintln!("마우스 이동 실패: {}", e);
            return;
        }

        self.total_moves += 1;
        self.last_move_time = Utc::now();
    }

    pub fn get_total_moves(&self) -> u64 {
        self.total_moves
    }

    pub fn get_last_move_time(&self) -> DateTime<Utc> {
        self.last_move_time
    }

    fn mouse_worker(is_running: Arc<AtomicBool>, settings: Settings) {
        while is_running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs_f64(settings.interval_seconds));

            if !is_running.load(Ordering::Relaxed) {
                break;
            }

            // 마우스 이동 수행
            if let Err(e) = Self::perform_mouse_move_internal(settings.move_distance) {
                eprintln!("마우스 이동 실패: {}", e);
            }
        }
    }

    fn perform_mouse_move(&self, distance: i32) -> Result<()> {
        Self::perform_mouse_move_internal(distance)
    }

    fn perform_mouse_move_internal(distance: i32) -> Result<()> {
        #[cfg(windows)]
        {
            use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

            unsafe {
                // 현재 마우스 위치에서 약간 이동
                mouse_event(MOUSEEVENTF_MOVE, distance as u32, 0, 0, 0);
                thread::sleep(Duration::from_millis(100));
                mouse_event(MOUSEEVENTF_MOVE, -(distance as i32) as u32, 0, 0, 0);
            }
        }

        #[cfg(unix)]
        {
            use std::ptr;
            use x11::xlib::*;

            unsafe {
                // X11 디스플레이 연결
                let display = XOpenDisplay(ptr::null());
                if display.is_null() {
                    return Err(anyhow::anyhow!("X11 디스플레이에 연결할 수 없습니다"));
                }

                // 현재 마우스 위치 가져오기
                let mut root_return = 0;
                let mut child_return = 0;
                let mut root_x = 0;
                let mut root_y = 0;
                let mut win_x = 0;
                let mut win_y = 0;
                let mut mask_return = 0;

                let root_window = XDefaultRootWindow(display);
                XQueryPointer(
                    display,
                    root_window,
                    &mut root_return,
                    &mut child_return,
                    &mut root_x,
                    &mut root_y,
                    &mut win_x,
                    &mut win_y,
                    &mut mask_return,
                );

                // 마우스를 약간 이동
                XWarpPointer(
                    display,
                    0,
                    root_window,
                    0,
                    0,
                    0,
                    0,
                    root_x + distance,
                    root_y,
                );
                XFlush(display);

                thread::sleep(Duration::from_millis(100));

                // 원래 위치로 돌아가기
                XWarpPointer(display, 0, root_window, 0, 0, 0, 0, root_x, root_y);
                XFlush(display);

                // 디스플레이 연결 해제
                XCloseDisplay(display);
            }
        }

        #[cfg(not(any(windows, unix)))]
        {
            // 지원되지 않는 플랫폼
            return Err(anyhow::anyhow!("지원되지 않는 플랫폼입니다"));
        }

        Ok(())
    }
}
