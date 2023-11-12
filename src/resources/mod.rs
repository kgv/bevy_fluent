//! Resources
//!
//! Any entity located directly in this module is
//! [`Resource`](bevy::ecs::system::Resource).

#[doc(inline)]
pub use self::fallback_chain::FallbackChain;

mod fallback_chain;
