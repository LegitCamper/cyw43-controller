//! Resident CYW43439 ISO controller patch images and their ABI.
//!
//! The CYW43439's stock `.0065` firmware reports no ISO support. These images
//! are installed into controller SRAM at startup so the controller answers
//! ordinary HCI ISO. They are generated and validated in the `cyw43_iso`
//! repository; this crate only carries the bytes, in the role `nrf-sdc-sys`
//! plays for Nordic parts.
//!
//! This crate installs nothing. It exposes the image bytes, the bootstrap
//! allocator, and the addresses and offsets an installer needs. Installation
//! order, verification and rollback belong to the host driver.
//!
//! # Constraints
//!
//! Every image targets exactly one firmware build, identified by
//! [`FIRMWARE`] and [`ROM_SHA256`]; installing against any other build is
//! undefined. The controller allocation is never reclaimed, so a failed or
//! cancelled installation requires a chip restart and firmware reload, and
//! HCI Reset neither reclaims the allocation nor reliably removes the hooks.
//! The stock test-radio event owns the radio while an image is active, so
//! concurrent normal controller operation is not established.
//!
//! This is a reverse-engineering experiment, not a qualified Bluetooth
//! implementation and not standard CIS/BIG negotiation.

#![no_std]

/// Controller this patch targets.
pub const CONTROLLER: &str = "CYW43439";

/// Bluetooth firmware build these images patch.
pub const FIRMWARE: &str = "0065";

/// SHA-256 of the clean controller ROM the images were derived from.
pub const ROM_SHA256: &str = "d5e271f20c80293be3561bb63e92ce4f6ca563ecca8fffd81eba639de9354038";

/// SHA-256 of the boot firmware that reserves the bootstrap's zero bytes.
pub const BOOT_FIRMWARE_SHA256: &str =
    "b3c0c99774135c32beea47af847838077a1f0b7a6be661a68ddcd3641362ad7d";

/// Manifest ABI version shared by every image here.
pub const ABI_VERSION: u32 = 1;

/// Address the bootstrap allocator is written to. Boot firmware must reserve
/// 112 zero bytes here, which [`BOOT_FIRMWARE_SHA256`] identifies.
pub const BOOTSTRAP_ADDRESS: u32 = 0x000d_0f90;

/// Guarded bootstrap that allocates resident SRAM through the controller's own
/// arena allocator. Shared by every image; the reservation size differs per
/// image and is patched by the installer.
pub const ALLOCATOR: &[u8; 112] = include_bytes!("../blobs/rx-stream/allocator.bin");

/// Controller pointers an installer must verify before publishing hooks.
pub mod pointers {
    /// Holds the transport context pointer.
    pub const TRANSPORT_CONTEXT: u32 = 0x0020_1f1c;
    /// Transport callback's offset within that context.
    pub const TRANSPORT_CALLBACK_OFFSET: u32 = 0x100;
    /// Stock transport callback; an installer must find this before taking over.
    pub const EXPECTED_TRANSPORT_CALLBACK: u32 = 0x0004_4105;
    /// Holds the radio callback table pointer.
    pub const RADIO_CALLBACK_TABLE: u32 = 0x0020_3a80;
    /// Stock radio callback table; an installer must find this before taking over.
    pub const EXPECTED_RADIO_CALLBACK_TABLE: u32 = 0x0008_b634;
    /// Cloned-table slot for the radio phase callback.
    pub const PHASE_TABLE_SLOT: usize = 4;
    /// Cloned-table slot for the receive callback.
    pub const RX_TABLE_SLOT: usize = 6;
}

/// Fixed transport parameters. The images do not negotiate these.
pub mod transport {
    /// ISO connection handle the images use.
    pub const HANDLE: u16 = 0x0042;
    /// SDU length in octets.
    pub const SDU_OCTETS: usize = 26;
    /// H4 packet indicator for ISO data.
    pub const HCI_PACKET_TYPE: u8 = 5;
}

/// Native ISO transmit with continuous discovery advertising.
#[cfg(feature = "continuous")]
pub mod continuous {
    /// Resident image.
    pub const IMAGE: &[u8; 4096] = include_bytes!("../blobs/continuous/native.bin");
    /// Bytes the installer must reserve.
    pub const ALLOCATION_BYTES: u32 = 4096;
}

/// One-shot fixed-channel receive, delivering a single SDU.
#[cfg(feature = "rx")]
pub mod rx {
    /// Resident image.
    pub const IMAGE: &[u8; 1024] = include_bytes!("../blobs/rx/native.bin");
    /// Bytes the installer must reserve.
    pub const ALLOCATION_BYTES: u32 = 1024;
}

/// Continuous fixed-channel receive through a rearmed single-slot mailbox.
///
/// The mailbox is lossy by construction: a busy mailbox drops newer packets.
/// Sequence numbers count accepted SDUs and are not derived from synchronised
/// BIG events, so they support no loss accounting.
#[cfg(feature = "rx-stream")]
pub mod rx_stream {
    /// Resident image.
    pub const IMAGE: &[u8; 1024] = include_bytes!("../blobs/rx-stream/native.bin");
    /// Bytes the installer must reserve.
    pub const ALLOCATION_BYTES: u32 = 1024;

    /// Fixed radio configuration this image receives on.
    pub mod radio {
        /// 2404 MHz, which is data channel 0.
        pub const FREQUENCY_MHZ: u32 = 2404;
        /// HCI test channel selecting that frequency.
        pub const HCI_TEST_CHANNEL: u8 = 1;
        /// Logical data channel used for whitening.
        pub const LOGICAL_CHANNEL: u8 = 0;
        /// BIS access address.
        pub const ACCESS_ADDRESS: u32 = 0x718e_bed6;
        /// BIS CRC initialiser.
        pub const CRC_INIT: u32 = 0x0012_3401;
        /// Raw controller header an accepted packet must present.
        pub const EXPECTED_RAW_HEADER: u32 = 0x1a00_1a00;
    }

    /// Offsets within the allocation, for reading diagnostics back.
    pub mod offsets {
        /// Receive callback wrapper.
        pub const RX_COMPLETE: u32 = 0;
        /// Cloned stock callback table.
        pub const CALLBACK_TABLE: u32 = 256;
        /// Leading guard word.
        pub const GUARD_BEFORE: u32 = 316;
        /// Snapshot record.
        pub const SNAPSHOT: u32 = 320;
        /// Captured payload within the snapshot.
        pub const PAYLOAD: u32 = 368;
        /// Trailing guard word.
        pub const GUARD_AFTER: u32 = 400;
        /// Radio phase wrapper.
        pub const PHASE: u32 = 416;
        /// Transport task bridge.
        pub const TRANSPORT: u32 = 576;
        /// Delivery state.
        pub const TASK_STATE: u32 = 736;
        /// Count of snapshots rejected by the task's guards.
        pub const REJECTED_SNAPSHOTS: u32 = 744;
        /// Radio configuration trace.
        pub const PHASE_TRACE: u32 = 768;
        /// HCI ISO emitter.
        pub const EMITTER: u32 = 800;
        /// Interrupt forwarding path.
        pub const ISR_FORWARD: u32 = 912;
    }

    /// Guard word written on both sides of the snapshot record.
    pub const GUARD_WORD: u32 = 0x5258_5331;
}
