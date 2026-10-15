# cyw43-controller-proto

Bluetooth LE isochronous link-layer and LE Audio byte formats, with no host
stack and no radio dependency, so the same code can run inside controller
firmware, inside test firmware on an independent radio, and on a host.

| Module | Contents |
| --- | --- |
| `channel` | Channel Selection Algorithm #2 for CIS/BIS events and subevents |
| `bis` | BIS Access Address and CRC initialiser derivation |
| `broadcast` | Cleartext BIGInfo and single-BIS BASE |
| `broadcast_advertising` | `ADV_EXT_IND`/`AUX_ADV_IND`/`AUX_SYNC_IND` with AuxPtr and SyncInfo |
| `isoal`, `isoal_framed` | Unframed and framed ISOAL segmentation and recombination |
| `ltv` | LC3 `Codec_Specific_Configuration` LTVs and unicast QoS bounds |

Sources are Bluetooth Core 6.0 and the Generic Audio Assigned Numbers,
cross-checked against Zephyr v4.2.0. Core sample vectors are used as tests
where the specification provides them, notably for CSA #2.

Encoding, decoding and derivation only: nothing here establishes a CIS or BIG,
schedules radio events, assigns stream sequence numbers, or recovers
synchronisation references. Not Bluetooth qualified.
