pub mod ratios;

pub mod prelude;
pub use prelude::*;

pub trait F64Extras {
    fn round_to(self, decimals: u32) -> f64;
    fn round_2(self) -> f64;
    fn round_4(self) -> f64;
}

impl F64Extras for f64 {
    fn round_to(self, decimals: u32) -> f64 {
        let factor = 10f64.powi(decimals as i32);
        (self * factor).round() / factor
    }

    fn round_2(self) -> f64 {
        self.round_to(2)
    }

    fn round_4(self) -> f64 {
        self.round_to(4)
    }
}
