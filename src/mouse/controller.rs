use crate::config::Settings;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use anyhow::Result;

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
        
        Ok(())
    }
} 