use std::time::{Duration, Instant};

pub struct Tween {
    pub servo_idx: usize,
    pub start: f32,
    pub target: f32,
    pub started: Instant,
    pub duration: Duration,
}

impl Tween {
    pub fn sample(&self, now: Instant) -> f32 {
        let mut p = now.duration_since(self.started).as_secs_f32() / self.duration.as_secs_f32();
        p = p.clamp(0.0, 1.0);

        if p < 1.0 {
            let eased = (3.0*p.powi(2)) - (2.0*p.powi(3));
            self.start + (self.target - self.start) * eased 
        } else {
            self.target
        }
    }

    pub fn is_finished(&self, now: Instant) -> bool {
        now.duration_since(self.started) >= self.duration
    }
}