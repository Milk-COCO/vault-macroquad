use std::time::{Duration, Instant};

/// 高精度计时器
pub struct Watch {
    current_duration: Duration,
    last_resume_instant: Option<Instant>,
    is_paused: bool,
}

impl Watch {
    pub fn new() -> Self {
        Self {
            current_duration: Duration::ZERO,
            last_resume_instant: None,
            is_paused: true,
        }
    }

    /// 获取当前时间（秒）
    pub fn time(&self) -> f64 {
        let total = if self.is_paused {
            self.current_duration
        } else {
            let now = Instant::now();
            let last = self.last_resume_instant.expect("Timer state corruption");
            self.current_duration + now.duration_since(last)
        };
        total.as_secs_f64()
    }

    /// 暂停并把播放进度设为目标值（秒）
    pub fn pause_seek_to(&mut self, time: f64) {
        self.current_duration = Duration::from_secs_f64(time.max(0.0));
        self.last_resume_instant = None;
        self.is_paused = true;
    }

    /// 播放进度设为目标值（秒）
    pub fn seek_to(&mut self, time: f64) {
        self.current_duration = Duration::from_secs_f64(time.max(0.0));
        // self.is_paused = false;
        self.last_resume_instant = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        if !self.is_paused {
            self._sync_logic_time();
            self.is_paused = true;
            self.last_resume_instant = None;
        }
    }

    pub fn resume(&mut self) {
        if self.is_paused {
            self.is_paused = false;
            self.last_resume_instant = Some(Instant::now());
        }
    }

    fn _sync_logic_time(&mut self) {
        if !self.is_paused {
            if let Some(last) = self.last_resume_instant {
                self.current_duration += Instant::now().duration_since(last);
                self.last_resume_instant = Some(Instant::now());
            }
        }
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}