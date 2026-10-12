# crowsi-rescue-console

Review emergency containment and recovery through a path independent of normal management.

## What you can do

- Check rescue-path readiness.
- Represent explicitly approved emergency operations.

## Current scope

An independently provisioned rescue path and recovery proof are required. The command does not grant emergency authority by itself.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-rescue-console
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-control](https://github.com/vpremises/vpremises-control). Use the [product README](../../README.md) for composition, use cases and trust boundaries.
