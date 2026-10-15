# crowsi-policy-engine

Decide whether a verified operation request satisfies an explicit policy.

## What you can do

- Evaluate identity, intent and supplied policy information.
- Return a reproducible allow or deny result.

## Current scope

The engine consumes verified inputs and returns a decision. It does not perform the requested operation.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-policy-engine
```

## Documentation and source

[Interface reference](docs/interface-reference.md)

[Usage guide](docs/getting-started.md)

[Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/authorized-execution](https://github.com/crowsi-net/authorized-execution). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
