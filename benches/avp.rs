//! avp benchmark
//!
//!  This particular module organization allows benchmarks to be completely hidden by the benchmarks feature.

#[cfg(feature = "benchmarks")]
mod avp_impl;

#[cfg(feature = "benchmarks")]
use criterion::criterion_main;

#[cfg(feature = "benchmarks")]
criterion_main!(avp_impl::benches);
