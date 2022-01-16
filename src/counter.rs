use time;

pub struct Counter {
    pub counter: time::Instant,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            counter: time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> time::Duration {
        time::Instant::now() - self.counter
    }

    pub fn reset(&mut self) {
        self.counter = time::Instant::now();
    }

    pub fn elapsed_gt(&self, msecs: i64) -> bool {
        self.elapsed() >= time::Duration::milliseconds(msecs)
    }
}
