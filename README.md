# dotenvy-workspace-example

Minimal example to investigate [dotenvy #74](https://github.com/allan2/dotenvy/issues/74)

- crate _a_ uses dotenvy_macro. It reproduces the error.
- crate _b_ uses dotenvy. It shows a solution.

## Issue

The problem is that the user expects the _.env_ inside the crate folder to be read. Instead, the workspace _.env_ is being read.

## Diagnosis
`dotenvy::dotenv` and the `dotenvy!` macro both read from `std::env::current_dir`.

If the user runs `cargo run --bin b` from the workspace root, then the current dir is the workspace root.

## Solution
To get the crate dir, we can use `CARGO_MANIFEST_DIR` and `dotenvy::from_path`.

```rust
let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
dotenvy::from_path(format!("{manifest_dir}/.env"))?;
```

## Will this be adopted?
No, because not all users use Cargo. Some just use the `rustc` compiler.

However, an example will be added to the repo.