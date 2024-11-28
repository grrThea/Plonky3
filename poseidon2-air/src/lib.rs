//! And AIR for the Poseidon2 permutation.

extern crate alloc;

mod air;
mod columns;
mod constants;
mod generation;
mod vectorized;

pub use air::*;
pub use columns::*;
pub use constants::*;
pub use generation::*;
pub use vectorized::*;
