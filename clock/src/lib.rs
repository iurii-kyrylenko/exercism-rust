use std::fmt;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    sys_time: SystemTime
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalize(hours * 60 + minutes),
            sys_time: SystemTime::now()
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalize(self.minutes + minutes),
            sys_time: self.sys_time
        }
    }

    pub fn add_minutes_mut(&mut self, minutes: i32) -> &Self {
        self.minutes = Clock::normalize(self.minutes + minutes);
        self
    }

    fn normalize(minutes: i32) -> i32 {
        const DAY_MINUTES: i32 = 24 * 60;

        (minutes % DAY_MINUTES + DAY_MINUTES) % DAY_MINUTES
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let minutes = (self.sys_time.elapsed().unwrap().as_secs() / 60) as i32;
        let minutes = Clock::normalize(self.minutes + minutes);

        write!(f, "{:02}:{:02}", minutes / 60, minutes % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}
