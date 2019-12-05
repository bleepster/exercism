use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    fn shorthand_tick(t: i32) -> i32 {
        if t == 23 {
            0
        } else {
            t + 1
        }
    }

    fn longhand_tick(t: i32) -> i32 {
        if t == 59 {
            0
        } else {
            t + 1
        }
    }

    fn move_hands(minutes: i32) -> (i32, i32) {
        let mut h: i32 = 0;
        let mut m: i32 = 0;

        for _ in 0..minutes.abs() {
            if m == 59 {
                h = Clock::shorthand_tick(h)
            }
            m = Clock::longhand_tick(m);
        }

        (h, m)
    }

    pub fn new(_hours: i32, _minutes: i32) -> Self {
        let _m = (_hours.rem_euclid(24) * 60 + _minutes).rem_euclid(24 * 60);
        Self { minutes: _m }
    }

    pub fn add_minutes(&self, _minutes: i32) -> Self {
        Self {
            minutes: (self.minutes + _minutes).rem_euclid(24 * 60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hours, minutes) = Clock::move_hands(self.minutes);
        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}
