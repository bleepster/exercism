// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    value: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { value: s }
    }
}

pub trait Planet {
    const ED_SECONDS: f64 = 31557600.0;
    const EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        let duration = d.value as f64;
        (duration / Self::ED_SECONDS) / Self::EARTH_YEARS
    }
}

macro_rules! planet {
    { $( $p: ident, $e: expr );* } => {
        $(
            pub struct $p;
            impl Planet for $p {
                const EARTH_YEARS: f64 = $e;
            }
        )*
    }
}

planet! {
    Mercury, 0.240847;
    Venus, 0.61519726;
    Earth, 1.0;
    Mars, 1.8808158;
    Jupiter, 11.862615;
    Saturn, 29.44749;
    Uranus, 84.016846;
    Neptune, 164.79132
}
