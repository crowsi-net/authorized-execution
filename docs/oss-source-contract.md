# OSS source contract: crowsi-net/authorized-execution

Reusable authorized execution components; product composition belongs to vPremises

## Responsibility boundary

Own the named security/transport mechanism and its explicit trust boundary. Product account/storage/UI composition belongs to the consumer. Tests must not substitute declarations for enforced authority or live environment acceptance.

Same-function managed packages take priority when their actual capabilities fit. Do not add an unrelated dependency merely to connect organizations. Preserve upstream LICENSE/NOTICE and immutable release/history evidence; unused compatibility sources belong in Git history, not the supported API.

## Package ownership

| Manifest | Package | Distribution policy |
| --- | --- | --- |
| `crates/crowsi-certificate-manager/Cargo.toml` | `crowsi-certificate-manager` | source-only; publish=false |
| `crates/crowsi-control-contracts/Cargo.toml` | `crowsi-control-contracts` | declared registry candidate; verify actual registry artifact |
| `crates/crowsi-enforcement-point/Cargo.toml` | `crowsi-enforcement-point` | source-only; publish=false |
| `crates/crowsi-independent-verifier/Cargo.toml` | `crowsi-independent-verifier` | source-only; publish=false |
| `crates/crowsi-key-provider/Cargo.toml` | `crowsi-key-provider` | source-only; publish=false |
| `crates/crowsi-local-control-bridge/Cargo.toml` | `crowsi-local-control-bridge` | declared registry candidate; verify actual registry artifact |
| `crates/crowsi-pa-pep-transport/Cargo.toml` | `crowsi-pa-pep-transport` | source-only; publish=false |
| `crates/crowsi-policy-administrator/Cargo.toml` | `crowsi-policy-administrator` | source-only; publish=false |
| `crates/crowsi-policy-engine/Cargo.toml` | `crowsi-policy-engine` | source-only; publish=false |
| `crates/crowsi-rescue-console/Cargo.toml` | `crowsi-rescue-console` | source-only; publish=false |

Manifest versions and publish flags are declarations, not proof of published availability. Public npm exports and Rust implementation/docs belong to the manifests above; generated dist and sibling checkouts are not source-of-truth implementations.

## Source completion criteria

1. The documented purpose, public exports, validators/schema and actual implementation agree. Remove only confirmed-unused aliases, schemas and compatibility adapters after checking consumers; a shared field name does not imply a shared trust responsibility.
2. Runtime and official dependency resolution work from an independent checkout with committed locks. No local sibling artifact, private registry or ambient owner credential is required for the OSS source path.
3. Supported behavior has reproducible examples/playground and functional tests, including invalid input and relevant failure/cancellation/replay/tenant boundaries. A policy-only CI or planning fixture does not prove external effects.
4. CI runs the declared functional acceptance without disguising skipped/unavailable checks as success. Changed public package bytes receive a new reviewed release; immutable old versions are never overwritten.
5. License/NOTICE, security reporting, usage and the actual supported/unsupported scope stay in the owning repository. The owner operates independently of a central private management checkout.
6. Source acceptance, artifact/registry acceptance and production deployment are separate records. This document defines the target; it does not declare unexecuted tests, unreleased APIs or live-account validation completed.

## Existing verification entrypoints

- [.github/workflows/ci.yml](../.github/workflows/ci.yml)

Use the package scripts/owner CI above and the repository README for exact commands. When schema files exist, their canonical validator and consumer must be checked together; deleting a definition on a text search alone is insufficient.

## Acceptance record

Inventory baseline source: `1f30eeb10047dcaea807e38ab786f4a3ffcede7c`. This baseline records observed files and declared interfaces, not a completed audit. Subsequent source changes are accepted at their exact commit with test evidence. Production hosting, account login, secret custody installation and data migration remain outside source-only acceptance.

## Owned source and acceptance map

[oss-source-map.json](oss-source-map.json) is the tracked map of package
interfaces, declared commands, schemas, functional test sources, examples and CI
jobs. It keeps each responsibility attached to its actual physical source owner.
Public exports are supported only within this repository's responsibility boundary;
a filename or schema declaration does not grant product or cloud authority.

| Package | Canonical manifest | Responsibility/interface |
| --- | --- | --- |
| `crowsi-certificate-manager` | [crates/crowsi-certificate-manager/Cargo.toml](../crates/crowsi-certificate-manager/Cargo.toml) | Fail-closed certificate lifecycle boundary for authenticated Crowsi workloads |
| `crowsi-control-contracts` | [crates/crowsi-control-contracts/Cargo.toml](../crates/crowsi-control-contracts/Cargo.toml) | Closed Zero Trust control contracts for Crowsi |
| `crowsi-enforcement-point` | [crates/crowsi-enforcement-point/Cargo.toml](../crates/crowsi-enforcement-point/Cargo.toml) | Fail-closed provider enforcement boundary for Crowsi |
| `crowsi-independent-verifier` | [crates/crowsi-independent-verifier/Cargo.toml](../crates/crowsi-independent-verifier/Cargo.toml) | Independent signed read-back and intrusion evidence verifier for Crowsi |
| `crowsi-key-provider` | [crates/crowsi-key-provider/Cargo.toml](../crates/crowsi-key-provider/Cargo.toml) | Hardware-backed signing boundary contracts for Crowsi |
| `crowsi-local-control-bridge` | [crates/crowsi-local-control-bridge/Cargo.toml](../crates/crowsi-local-control-bridge/Cargo.toml) | Authenticated Unix IPC boundary for local Crowsi control |
| `crowsi-pa-pep-transport` | [crates/crowsi-pa-pep-transport/Cargo.toml](../crates/crowsi-pa-pep-transport/Cargo.toml) | Versioned local PA-to-PEP composition boundary for Crowsi |
| `crowsi-policy-administrator` | [crates/crowsi-policy-administrator/Cargo.toml](../crates/crowsi-policy-administrator/Cargo.toml) | Signed authorization verification and durable one-use enforcement ledger for Crowsi |
| `crowsi-policy-engine` | [crates/crowsi-policy-engine/Cargo.toml](../crates/crowsi-policy-engine/Cargo.toml) | Pure deterministic Zero Trust policy evaluation for Crowsi |
| `crowsi-rescue-console` | [crates/crowsi-rescue-console/Cargo.toml](../crates/crowsi-rescue-console/Cargo.toml) | Out-of-band containment and separately approved recovery authority |

For source acceptance, run each applicable functional CI job with the committed
locks from an independent checkout. Match public entrypoints to their test/example
sources in the map; test the relevant invalid input and failure boundary as well as
the normal path. Policy-only, skipped and credential-dependent jobs do not establish
functional coverage. Release jobs establish artifact acceptance only when actually
run at the reviewed commit. A source-only/example repository must not be presented
as a complete external service or a released package.

Schema definitions in the map remain owned by their canonical validator and
consumers. An absent schema file means no separate schema file is declared; it does
not mean the runtime interface has no validation. Do not introduce another common
schema solely to mirror an implementation type.
