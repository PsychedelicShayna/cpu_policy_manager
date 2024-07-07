use anyhow as ah;

enum Frequency {
    GHz(u32),
    MHz(u32),
    KHz(u32),
    Hz(u32),
}

impl Frequency {
    pub fn into_hz(&self) -> u32 {
        match self {
            Frequency::GHz(ghz) => ghz * 1_000_000_000,
            Frequency::MHz(mhz) => mhz * 1_000_000,
            Frequency::KHz(khz) => khz * 1_000,
            Frequency::Hz(hz) => *hz,
        }
    }

    pub fn into_khz(&self) -> Frequency {
        Frequency::KHz(self.into_hz() / 1_000)
    }

    pub fn into_mhz(&self) -> Frequency {
        Frequency::MHz(self.into_hz() / 1_000_000)
    }

    pub fn into_ghz(&self) -> Frequency {
        Frequency::GHz(self.into_hz() / 1_000_000_000)
    }
}

// Writes the frequency to every CPU core policy.
pub fn set_freq(_frequency: u32) -> ah::Result<()> {


    todo!();
}
