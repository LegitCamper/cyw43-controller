//! Bluetooth LE isochronous link-layer and LE Audio byte formats.
//!
//! These are the formats a controller must produce or consume on air, kept
//! free of any host stack so the same code can run inside controller firmware,
//! inside test firmware on an independent radio, and on a host.
//!
//! Scope is encoding, decoding and derivation only. Nothing here establishes a
//! CIS or BIG, schedules radio events, assigns stream sequence numbers, or
//! recovers synchronisation references; those belong to whatever owns the
//! radio. Successfully encoding a value is never evidence of a corresponding
//! on-air event.
//!
//! Sources are Bluetooth Core 6.0 and the Generic Audio Assigned Numbers,
//! cross-checked against Zephyr v4.2.0. None of this is Bluetooth qualified.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod bis;
pub mod broadcast;
pub mod broadcast_advertising;
pub mod channel;
pub mod isoal;
pub mod isoal_framed;
pub mod ltv;
