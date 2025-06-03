#[cfg(all(feature = "v38", feature = "v42"))]
compile_error!("Cannot use more than one version feature at a time.");

#[cfg(feature = "v42")]
#[path = "v42/mod.rs"]
mod v42;

#[cfg(feature = "v38")]
#[path = "v38/mod.rs"]
mod v38;

#[cfg(feature = "v42")]
pub use v42::*;

#[cfg(feature = "v38")]
pub use v38::*;