use std::time::Instant;

pub struct TimedProcess {
    pub start_time: Instant,
    pub name: String,
}

impl TimedProcess {
    pub fn start(name: &str) -> TimedProcess {
        println!("Process: {}, has started", name);

        TimedProcess {
            start_time: Instant::now(),
            name: name.to_string(),
        }
    }

    pub fn stop(&self) {
        println!("Process: {}, has stopped", self.name);
        println!("Time elapsed: {:?}", self.start_time.elapsed());
    }
}
