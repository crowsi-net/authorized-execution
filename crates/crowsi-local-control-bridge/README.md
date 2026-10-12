# crowsi-local-control-bridge

Authenticate a local process and authorize its request before passing it to a protected service.

## What you can do

- Bind a Unix-stream request to peer and workload evidence.
- Validate exact action, resource and purpose.

## Current scope

The host supplies trust, policy and private socket placement. A reachable socket is not sufficient authorization.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-local-control-bridge
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-control](https://github.com/vpremises/vpremises-control). Use the [product README](../../README.md) for composition, use cases and trust boundaries.

## Durable security state

The bridge uses SQLite v4 with durable status anchors, revocation watermarks,
one-use reservations and audit records. Legacy v2/v3 state is rejected without
implicit migration or mutation; operators must make an explicit state transition
decision. An unrecognized schema never becomes trusted current state.
