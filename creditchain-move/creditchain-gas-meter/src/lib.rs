// Copyright © A-p-t-o-s Foundation
// SPDX-License-Identifier: Apache-2.0

//! This crate serves as the implementation of the standard gas meter and algebra used in the CreditChain VM.
//! It also defines traits that enable composability of gas meters and algebra.

mod algebra;
mod meter;
mod traits;

pub use algebra::StandardGasAlgebra;
pub use meter::StandardGasMeter;
pub use traits::{CreditChainGasMeter, GasAlgebra};
