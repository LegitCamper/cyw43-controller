# cyw43-controller

Bluetooth LE isochronous support for the CYW43439, in the shape `nrf-sdc` uses
for Nordic parts: a vendor-specific controller behind Rust crates.

The CYW43439's stock `.0065` firmware reports no ISO support. These crates
reach it through a resident patch installed into controller SRAM at startup,
so the controller answers ordinary HCI ISO. This is a reverse-engineering
experiment against one firmware build, not a qualified Bluetooth
implementation and not standard CIS/BIG negotiation.

## Crates

| Crate | Role |
| --- | --- |
| [`cyw43-controller`](cyw43-controller) | Safe startup installer and lifecycle state machine. Bus-driver independent, analogous to `nrf-sdc`. |
| [`cyw43-controller-blob`](cyw43-controller-blob) | Resident patch bytes plus build-time ABI digest verification, analogous to `nrf-sdc-sys`. |
| [`cyw43-controller-proto`](cyw43-controller-proto) | Controller/link-layer byte formats: CSA#2, BIS, BIG advertising, ISOAL and LTV. No host stack. |

`cyw43_iso` remains the reverse-engineering and blob-generation project.
Embassy or another bus owner implements `cyw43_controller::Runtime` with four
exclusive-startup operations; controller policy and patch lifecycle do not
live in that host driver.

Consumers are expected to be `trouble_audio` for LE Audio profiles,
`rp2040_audio` for Pico W applications, and the independent nRF54L15 test
firmware that validates the radio.

## Scope

`proto` encodes, decodes and derives. It does not establish a CIS or BIG,
schedule radio events, assign sequence numbers, or recover synchronisation
references; those belong to controller implementation. Encoding a value is
never evidence of a corresponding on-air event.

All crates forbid unsafe code and build on `thumbv6m-none-eabi`,
`thumbv8m.main-none-eabihf` and host. `proto` is `no_std` with safe `alloc` by
default; allocator-free users can disable its default feature.
