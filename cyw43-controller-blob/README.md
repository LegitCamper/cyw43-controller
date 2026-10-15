# cyw43-controller-blob

Resident CYW43439 ISO controller patch images, matching boot firmware and their
ABI, in the role `nrf-sdc-sys` plays for Nordic parts.

The stock `.0065` Bluetooth firmware reports no ISO support. These images are
installed into controller SRAM at startup so the controller answers ordinary
HCI ISO. They are generated and validated in `cyw43_iso`; this crate only
carries the bytes and the addresses an installer needs.

`BOOT_FIRMWARE` is the tested `.0065` HCD container with one 112-byte zeroed
record at `0x000d0f90`. Load it instead of a separate stock `btfw` before
installing any image. All resident images share this one boot firmware; it is
not duplicated per image.

| Feature | Image | Size | Behaviour |
| --- | --- | --- | --- |
| `continuous` | `continuous` | 4096 | Native ISO transmit with continuous discovery advertising |
| `rx` | `rx` | 1024 | One-shot fixed-channel receive, one SDU |
| `rx-stream` | `rx-stream` | 1024 | Continuous fixed-channel receive through a rearmed single-slot mailbox |
| `rx-hop` | `rx-hop` | 2048 | Open-loop CSA#2 receive, advanced by matching RX callbacks |
| `rx-timer-hop` | `rx-timer-hop` | 2048 | Fast pre-acquisition CSA#2 scan, then marker-aligned timer hopping |

This crate installs nothing: installation order, verification and rollback
belong to the host driver.

## Integrity

`build.rs` checks every shipped image and the boot firmware against the byte
size and SHA-256 in their manifests, so a corrupted or hand-edited binary fails
the build instead of reaching a controller. Regenerate them in `cyw43_iso`
rather than editing them here.

`tests/abi.py` checks the hand-written constants in `src/lib.rs` against the
generated manifests, rejecting drift in identity, bootstrap address, controller
pointers, transport parameters, radio configuration, resident offsets and
per-image allocation sizes.

## Constraints

Each image targets exactly one firmware build, identified by `FIRMWARE` and
`ROM_SHA256`; installing against another build is undefined. The controller
allocation is never reclaimed, so a failed or cancelled installation requires a
chip restart and firmware reload, and HCI Reset neither reclaims it nor reliably
removes the hooks. The stock test-radio event owns the radio while an image is
active, so concurrent normal controller operation is not established.

Radio and transport parameters are fixed, not negotiated: one unencrypted BIS,
mono 8 kHz, 10 ms SDUs of 26 octets on handle `0x42`. The `rx-stream` mailbox is
lossy by construction, a busy mailbox drops newer packets, and its sequence
numbers count accepted SDUs rather than synchronised BIG events, so they support
no loss accounting. Both hopping images lack a BIGInfo-derived anchor. The
timer image scans one event candidate per accepted countdown callback before
acquisition; its callback spacing and synchronized 20-ms cadence remain nominal
until measured on hardware.

Reverse-engineering experiment; not qualified, and not standard CIS/BIG
negotiation.

## Binary licensing

Rust source remains `MIT OR Apache-2.0`. The boot firmware derives from Pico
SDK's `lib/cyw43-driver/firmware/cyw43_btfw_43439.h` and is separately covered
by `licenses/LICENSE` and `licenses/LICENSE.RP`. The RP license permits modified
redistribution only with Raspberry Pi semiconductor devices; the other license
limits use to personal, non-commercial purposes. Do not treat the firmware as
MIT or Apache licensed.
