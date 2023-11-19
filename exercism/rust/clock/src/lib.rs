use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
            let mut total_minutes = hours * 60 + minutes;
            let one_day_in_minutes = 24 * 60;
            if total_minutes < 0 {
                total_minutes = total_minutes % one_day_in_minutes + one_day_in_minutes;
            } else if total_minutes >= one_day_in_minutes {
                total_minutes = total_minutes % one_day_in_minutes;
            }

            Clock {
                hours: total_minutes / 60,
                minutes: total_minutes % 60,
            }
        }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        Clock::new(0, total_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
