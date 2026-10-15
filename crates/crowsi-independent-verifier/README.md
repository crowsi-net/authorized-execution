# crowsi-independent-verifier

Check an operation result against evidence obtained independently of its execution receipt.

## What you can do

- Match identity, scope and expected read-back evidence.
- Return verification results without executing the operation.

## Current scope

The caller supplies trusted read-back inputs. Missing evidence remains unresolved rather than being assumed successful.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-independent-verifier
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/authorized-execution](https://github.com/crowsi-net/authorized-execution). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
