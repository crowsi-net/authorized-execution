# authorized-execution

Reusable Crowsi components with independent trust roles. Product registration, CLI composition and installed native helpers belong to [vPremises environment operations](https://github.com/vpremises/environment-operations).

## Packages

- [crowsi-certificate-manager](crates/crowsi-certificate-manager): existing crate identity and publication declaration retained.
- [crowsi-control-contracts](crates/crowsi-control-contracts): existing crate identity and publication declaration retained.
- [crowsi-enforcement-point](crates/crowsi-enforcement-point): existing crate identity and publication declaration retained.
- [crowsi-independent-verifier](crates/crowsi-independent-verifier): existing crate identity and publication declaration retained.
- [crowsi-key-provider](crates/crowsi-key-provider): existing crate identity and publication declaration retained.
- [crowsi-local-control-bridge](crates/crowsi-local-control-bridge): existing crate identity and publication declaration retained.
- [crowsi-pa-pep-transport](crates/crowsi-pa-pep-transport): existing crate identity and publication declaration retained.
- [crowsi-policy-administrator](crates/crowsi-policy-administrator): existing crate identity and publication declaration retained.
- [crowsi-policy-engine](crates/crowsi-policy-engine): existing crate identity and publication declaration retained.
- [crowsi-rescue-console](crates/crowsi-rescue-console): existing crate identity and publication declaration retained.

## Verify and explore

```sh
cargo +1.97 test --locked --workspace --all-targets --all-features
cargo +1.97 test --locked --workspace --doc --all-features
```

Each crate README and examples describe its public API. Tests and examples use synthetic state; they do not install an authority, service or maintenance timer. Authority, broker, enforcement, independent verification and recovery retain separate roles. Existing wire/schema identifiers and publish=false declarations are unchanged. Repository relocation does not imply a new registry release. Original public history and license notices are preserved in docs/source-history.

[OSS responsibility and source completion](docs/oss-source-contract.md) defines the supported package boundary and required acceptance. It does not declare unexecuted source, distribution or production checks complete.

Dependency updates: [repository policy](docs/dependency-updates.md).
Registry releases: [version tags and GitHub OIDC](docs/registry-release.md).
