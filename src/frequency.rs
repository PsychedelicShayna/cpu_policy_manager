use std::ops::Div;

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    GHz(f64),
    MHz(u64),
    KHz(u64),
    Hz(u64),
}

// Convenient type aliases.
pub type Frq = Frequency;
pub type Freq = Frequency;

impl Frequency {
    pub fn to_hz(&self) -> Frequency {
        match self {
            Frequency::GHz(ghz) => Frequency::Hz((ghz * 1_000_000_000.0) as u64),
            Frequency::MHz(mhz) => Frequency::Hz(mhz * 1_000_000),
            Frequency::KHz(khz) => Frequency::Hz(khz * 1_000),
            hz @ Frequency::Hz(_) => *hz,
        }
    }

    pub fn to_khz(&self) -> Frequency {
        match self {
            Frequency::GHz(ghz) => Frequency::KHz((ghz * 1_000_000.0) as u64),
            Frequency::MHz(mhz) => Frequency::KHz(mhz * 1_000),
            Frequency::Hz(hz) => Frequency::KHz(hz / 1_000),
            khz @ Frequency::KHz(_) => *khz,
        }
    }

    pub fn to_mhz(&self) -> Frequency {
        match self {
            Frequency::GHz(ghz) => Frequency::MHz((ghz * 1_000.0) as u64),
            Frequency::KHz(khz) => Frequency::MHz(khz / 1_000),
            Frequency::Hz(hz) => Frequency::MHz(hz / 1_000_000),
            mhz @ Frequency::MHz(_) => *mhz,
        }
    }

    pub fn to_ghz(&self) -> Frequency {
        match self {
            Frequency::MHz(mhz) => Frequency::GHz((*mhz as f64) / 1_000.0),
            Frequency::KHz(khz) => Frequency::GHz((*khz as f64) / 1_000_000.0),
            Frequency::Hz(hz) => Frequency::GHz((*hz as f64) / 1_000_000_000.0),
            ghz @ Frequency::GHz(_) => *ghz,
        }
    }

    pub fn to_string_u64(&self) -> String {
        u64::from(*self).to_string()
    }

    pub fn to_string_f64(&self) -> String {
        f64::from(*self).to_string()
    }
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::GHz(ghz) => write!(f, "{:.2} GHz", ghz),
            Frequency::MHz(mhz) => write!(f, "{} MHz", mhz),
            Frequency::KHz(khz) => write!(f, "{} KHz", khz),
            Frequency::Hz(hz) => write!(f, "{} Hz", hz),
        }
    }
}

impl From<Frequency> for f64 {
    fn from(value: Frequency) -> Self {
        match value {
            Frequency::GHz(ghz) => ghz,
            Frequency::MHz(mhz) => mhz as f64,
            Frequency::KHz(khz) => khz as f64,
            Frequency::Hz(hz) => hz as f64,
        }
    }
}

impl From<Frequency> for u64 {
    fn from(value: Frequency) -> Self {
        match value {
            Frequency::GHz(ghz) => ghz as u64,
            Frequency::MHz(mhz) => mhz,
            Frequency::KHz(khz) => khz,
            Frequency::Hz(hz) => hz,
        }
    }
}
