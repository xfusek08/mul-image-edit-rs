
use std::time::{Instant, Duration};

#[derive(Clone)]
pub struct Tick {
    pub scheduled_time: Instant,
    pub current_time: Instant,
    pub delta: Duration,
}

impl Tick {
    pub fn now() -> Self {
        Self {
            scheduled_time: Instant::now(),
            current_time: Instant::now(),
            delta: Duration::ZERO,
        }
    }
    
    #[inline]
    pub fn tick(&self) -> Self {
        let current_time = Instant::now();
        let delta = current_time - self.current_time;
        Self { current_time, delta, ..*self }
    }
    
    #[inline]
    pub fn schedule(&self, scheduled_time : Instant) -> Self {
        Self { scheduled_time, ..*self }
    }
    
    #[inline]
    pub fn schedule_milis(&self, millis : u64) -> Self {
        self.schedule(Instant::now() + Duration::from_millis(millis))
    }
    
    #[inline]
    pub fn is_scheduled(&self) -> bool {
        self.current_time >= self.scheduled_time
    }
}


pub trait Tickable {
    fn tick(&mut self, tick: &crate::data::Tick) { }
}
