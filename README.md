# JSON Web Token Validation

This code provides a minimal example for using RISC Zero's [zkVM] to create zero-knowledge proofs that confirm the integrity of a signed JSON Web Token (JWT). It leverages the [jwt-compact] crate to verify the RS256 JWT signature and claim within the [zkVM].

## Quick Start

First, follow the [examples guide] to install dependencies and check out the correct version of the example.

Then, run the example with:

```bash
cargo run --release
```

Congratulations! You've successfully constructed a zero-knowledge proof attesting to the validity of an issued JWT, verified against a public key.

## Project Organization

zkVM applications consist of a [host program] and a [guest program]. The host program resides in [`src/main.rs`], and the guest program is in [`methods/guest/src/main.rs`]. The foundational JWT issuing/validation library is located in [`core/src/lib.rs`].

The [host] executes the guest program, then proves this execution to generate a [receipt]. This receipt, when presented to a third party, allows them to inspect the [journal] for the program's outputs and verify the receipt, confirming the [guest program]'s execution integrity. In this instance, the JWT's integrity is validated against a public key in the guest, and the decoded token subject is committed to the [journal] for demonstration.

## Approach

This zkVM application showcases how to utilize existing Rust crates and libraries. It employs the [jwt-compact] crate for issuing and validating JWTs using RSA key pairs. This process is encapsulated in a higher-level JWT crate to distinctly demonstrate symmetric key verification.

For more insights into using Rust crates within the zkVM, visit our [Rust Resources] page.

The [guest code] confirms the JWT's integrity using the public key and records the decoded JWT claim subject if successful. Meanwhile, the [host code] is responsible for generating and signing the JWT, including the claim subject string, with the private key.

