# Control boundary playground

Run from this owner repository, without a CoelaRelease or neighboring source checkout:

```sh
cargo run --locked -p crowsi-policy-engine --example policy_decisions
cargo test --locked -p crowsi-enforcement-point --test v2_failure_semantics --test fence_consumption --test adapter_boundary
cargo test --locked -p crowsi-independent-verifier
cargo test --locked -p crowsi-rescue-console
```

The runnable example calls the public policy API with checked-in synthetic contract fixtures. It demonstrates a permit decision, revocation and missing-evidence denial. Its fixture signatures are synthetic. Policy evaluation is pure; it does not authenticate signatures, mint authorization or execute an effect. The separate enforcement tests exercise signed command/lease binding, replay/failure and injected fake effect adapters. The verifier requires separate read-back evidence. Recovery uses a distinct trust path. None of these checks installs an authority, invokes a real cloud API or proves Windows CNG custody.

## Responsibility and distribution

The current workspace has separate crates for contracts, policy, signed authorization, transport, enforcement, verification and recovery. Contracts have several direct crate consumers, so they cannot be deleted merely because their name is generic. Schema versions represent distinct signed wire formats; review their validators, canonical bytes, producer/consumer and persisted ledger before eliminating any compatibility path. Do not weaken unknown-field, signature, expiry, target binding or replay checks.

The workspace remains at its current owner while final physical placement is reviewed. `publish = false` is intentional at this stage; a source CI pass is not a registry release. Libraries and native helper executables require separate distributions. Node in-memory control is not an interchangeable backend for signed, durable native authorization.

## Activation and recovery record

Before activation, the environment owner supplies an authority/operator identity, allowed account/resource/action, signed inputs and independently acquired read-back. Record exact source/archive/native digest, platform/ABI, backend, trust anchors, ledger and backup/restore method. Missing backend or signed input is a blocker, not an implicit fallback. Capture unknown/partial outcomes and revoke the exact grant; never retry a potentially executed operation by minting an unrestricted replacement. Recovery credentials and execution path must remain separate from the normal authority. Complete isolated Windows/WSL acceptance before using a Windows helper in an environment.

[日本語](README.ja.md)
