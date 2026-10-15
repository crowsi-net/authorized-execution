# crowsi-key-provider

Request a limited signature from a key provider that does not export the private key.

## What you can do

- Bind signing to a declared purpose and digest.
- Use a closed provider interface for protected keys.

## Current scope

Platform backends must be provisioned and verified separately. Defining a provider port does not prove that hardware custody is available.

Package distribution is not activated by this documentation. Use the owning product workspace and declared dependency versions. Registry availability is a separate release gate.

## Getting started

Install Rust 1.97 or newer. Run from the owning product workspace:

```sh
cargo test --locked -p crowsi-key-provider
```

## Documentation and source

[Usage guide](docs/getting-started.md)

[Schemas](schemas) · [Implementation and public interfaces](src) · [Verification cases](tests) · [Contributing](CONTRIBUTING.md) · [Security reporting](SECURITY.md) · [License](LICENSE) · [Attribution notices](NOTICE)

## Product responsibility

This component is maintained in [crowsi-net/authorized-execution](https://github.com/crowsi-net/authorized-execution). Use the [repository README](../../README.md) for composition, use cases and trust boundaries.
