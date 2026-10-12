# crowsi-control-contracts

Exchange control requests, decisions and execution evidence through a shared, strict contract.

## What you can do

- Use consistent typed control envelopes.
- Reject unsupported request shapes before policy evaluation.

## Current scope

These are contracts; authorization decisions and execution are supplied by their respective implementations.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-control-contracts
```

## Examples and interface details

## Certificate V2 contracts

Certificate lifecycle contracts are a separate closed family. They define six
non-interchangeable actions: issue, renew, revoke, certificate status,
operation status, and reconciliation of an unknown result. A signed execution
authorization binds the exact service/workload, canonical resource, action,
lease, revision, fence, identity revocation epoch, certificate lifecycle epoch,
policy ancestry, and—where required—independent operator-approval ancestry.

PA release and manager acceptance use
`CertificateManagerHandoffEvidenceV2`. Release alone does not authorize a CA
call. The manager first persists the exact claim, signs it with the
`certificate-manager-handoff-receipt` role, and waits for the PA's exact
durable ACK. Certificate outcome evidence and manager durable-commit evidence
remain separate artifacts and keys. The commit contract signs its event time,
evidence issuance time, short transport expiry, and bounded submission
recovery deadline so delayed first delivery cannot silently extend execution
authority.

These pure types do not enforce replay, trusted time, current revocation,
cryptographic low-S verification, database atomicity, or key uniqueness.
`crowsi-policy-administrator`, `crowsi-certificate-manager`, and
`crowsi-key-provider` must enforce those properties at their respective
boundaries. A SPIFFE-shaped subject remains only a policy identifier; these
contracts do not implement the SPIFFE Workload API.

`EnforcementGrantV1` and `RecoveryAuthorizationV1` fix `use_limit` to one and
carry unique JTIs. V2 also carries a one-use release reservation and monotonic
fence. A PA and PEP must still consume these atomically; pure contract
validation cannot prevent reuse across processes.

Coverage is fail-closed. Partial, unknown, stale, or unmanaged evidence cannot
be represented as verified or healthy. Receipts describe executor claims only;
they do not establish verification. A separate, independent coverage assertion
is required. Isolation readiness additionally requires a verified management
lifeline, a ready enforcer, and ready quarantine, revoke, verify, and restore
capabilities.

The JSON Schemas in `schemas/` are standalone Draft 2020-12 documents. New
incompatible fields or semantics require a new schema URI and Rust type.
Non-secret command, execution-lease, and receipt examples are in `fixtures/`;
their detached signatures are structural placeholders and are not
production-authentic. `cargo run --offline --example render_v2_fixtures`
regenerates equivalent sample output without contacting a provider.

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Examples](examples) · [Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-control](https://github.com/vpremises/vpremises-control). Use the [product README](../../README.md) for composition, use cases and trust boundaries.
