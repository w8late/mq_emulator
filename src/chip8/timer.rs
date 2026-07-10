use std::{sync::{Arc, atomic::{AtomicU8, Ordering}}, thread, time::Duration};

// timer that runs in a background thread
pub struct CountdownTimer<const HZ: u64>(Arc<AtomicU8>);

impl<const HZ: u64> CountdownTimer<HZ> {
    pub fn new() -> Self {
        Self (Arc::new(AtomicU8::new(0)))
    }

    pub fn with_thread() -> Self {
        let mut timer = CountdownTimer::new();
        timer.spawn_worker_thread();
        timer
    }

    fn spawn_worker_thread(&mut self) {
        let value = Arc::clone(&self.0);

        thread::spawn(move || loop {
            let _ = value.try_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                if v > 0 {
                    Some(v - 1)
                } else {
                     Some(v) 
                } 
            });
            thread::sleep(Duration::from_millis(1000 / HZ as u64));
        });
    }
    
    pub fn set(&mut self, new_time: u8) {
        self.0.store(new_time, Ordering::Relaxed); 
    }

    pub fn get(&self) -> u8 {
        self.0.load(Ordering::Relaxed)
    }

    #[allow(dead_code)]
    pub fn is_not_zero(&self) -> bool {
        self.get() > 0 
    }
}