# crowsi-pa-pep-transport

Connect a policy administrator to an enforcement point through a validated control exchange.

## What you can do

- Carry the declared administration/enforcement envelopes.
- Apply bounded transport checks.

## Current scope

The deployment supplies peer trust and authorized endpoints. Transport delivery does not grant an operation.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-pa-pep-transport
```

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [vpremises/vpremises-control](https://github.com/vpremises/vpremises-control). Use the [product README](../../README.md) for composition, use cases and trust boundaries.
