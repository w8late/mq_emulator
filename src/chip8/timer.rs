use std::{sync::{Arc, Mutex}, thread, time::Duration};

// timer that runs in a background thread
pub struct CountdownTimer<const HZ: u64>(Arc<Mutex<u8>>);

impl<const HZ: u64> CountdownTimer<HZ> {
    pub fn new() -> Self {
        Self (Arc::new(Mutex::new(0)))
    }

    pub fn with_thread() -> Self {
        let mut timer = CountdownTimer::new();
        timer.spawn_worker_thread();
        timer
    }

    fn spawn_worker_thread(&mut self) {
        let value = Arc::clone(&self.0);

        thread::spawn(move || {
            loop {
                let mut lock = value.lock().unwrap();
                if *lock > 0 {
                    *lock -= 1;
                }
                drop(lock);
                thread::sleep(Duration::from_millis(1000 / HZ as u64));
            }
        });
    }
    
    pub fn set_time(&mut self, new_time: u8) {
        *self.0.lock().unwrap() = new_time; 
    }

    pub fn get_time(&self) -> u8 {
        *self.0.lock().unwrap()
    }

    pub fn is_not_zero(&self) -> bool {
        *self.0.lock().unwrap() > 0 
    }
}