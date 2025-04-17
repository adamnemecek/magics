mod centered;
mod half_beginning_half_end;
mod interleave_evenly;
mod late_as_possible;
mod soon_as_possible;

// use std::num::NonZeroUsize;

pub use centered::*;
pub use half_beginning_half_end::*;
pub use interleave_evenly::*;
pub use late_as_possible::*;
pub use soon_as_possible::*;

// pub struct GbpScheduleTimestep(u64);

// trait CheckBit {
//     fn bit_n_set(self, n: u8) -> bool;
// }

// macro_rules! impl_check_bit {
//     ($T:ty, $n_bits:expr) => {
//         impl CheckBit for $T {
//             fn bit_n_set(self, n: u8) -> bool {
//                 debug_assert!(n <= $n_bits);
//                 (self & (1 << n)) > 0
//             }
//         }
//     };
// }

// impl_check_bit!(u8, 8);
// impl_check_bit!(u16, 16);
// impl_check_bit!(u32, 32);
// impl_check_bit!(u64, 64);
// impl_check_bit!(u128, 128);
// // impl_check_bit!(usize, ...);

// impl GbpScheduleTimestep {
//     pub fn on(&self, ix: u8) -> bool {
//         u64::bit_n_set(self.0, ix)
//     }
// }

// pub enum GbpScheduleTimestepError {
//     NumTimersExceedLimitOf64,
// }

// impl TryFrom<&[bool]> for GbpScheduleTimestep {
//     type Error;

//     fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// pub trait GbpScheduleIter<'a>: std::iter::Iterator<Item = u64> {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GbpScheduleAtIteration {
    pub internal: bool,
    pub external: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct GbpScheduleParams {
    pub internal: u8,
    pub external: u8,
}

impl GbpScheduleParams {
    pub(crate) fn max(self) -> u8 {
        self.internal.max(self.external)
    }
}

pub trait GbpScheduleIterator: std::iter::Iterator<Item = GbpScheduleAtIteration> {}

pub trait GbpSchedule {
    fn schedule(params: GbpScheduleParams) -> impl GbpScheduleIterator;
}
