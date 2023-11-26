// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[macro_export]
macro_rules! planet_years {
    ($s:expr,$r:expr) => {
        {
           $s as f64 / (31_557_600 as f64 * $r as f64)
        }
    };
}

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            seconds: s
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 1.0)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 0.2_408_467)        
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 0.61_519_726)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 1.0)
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 1.8_808_158)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 11.862_615)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 29.447_498)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 84.016_846)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        planet_years!(d.seconds, 164.79_132)
    }
}
