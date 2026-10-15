# cyw43-controller-blob

Resident CYW43439 ISO controller patch images and their ABI, in the role
`nrf-sdc-sys` plays for Nordic parts.

The stock `.0065` Bluetooth firmware reports no ISO support. These images are
installed into controller SRAM at startup so the controller answers ordinary
HCI ISO. They are generated and validated in `cyw43_iso`; this crate only
carries the bytes and the addresses an installer needs.

| Feature | Image | Size | Behaviour |
| --- | --- | --- | --- |
| `continuous` | `continuous` | 4096 | Native ISO transmit with continuous discovery advertising |
| `rx` | `rx` | 1024 | One-shot fixed-channel receive, one SDU |
| `rx-stream` | `rx-stream` | 1024 | Continuous fixed-channel receive through a rearmed single-slot mailbox |

This crate installs nothing: installation order, verification and rollback
belong to the host driver.

## Integrity

`build.rs` re-hashes every shipped image against the SHA-256 in its own
generated manifest, so a corrupted or hand-edited image fails the build instead
of reaching a controller. Regenerate images in `cyw43_iso` rather than editing
them here.

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
no loss accounting.

Reverse-engineering experiment; not qualified, and not standard CIS/BIG
negotiation.
