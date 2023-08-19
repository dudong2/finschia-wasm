# finschia-wasm

Rust libraries for Finschia. The following table shows every published crates maintained in this repository:

| Package                                             | Description                                                                                                                                                            |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [finschia-std](packages/finschia-std)               | Finschia's proto-generated types and helpers for interacting with the appchain. Compatible with CosmWasm contract.                                                      |
| [finschia-std-derive](packages/finschia-std-derive) | Procedural macro for augmenting proto-generated types to create better developer ergonomics. Internally used by `finschia-std`                                          |
| [proto-build](packages/proto-build) | Autogenerate rust types from Finschia's proto. This makes up the types of `finschia-std`.                                          |

---
