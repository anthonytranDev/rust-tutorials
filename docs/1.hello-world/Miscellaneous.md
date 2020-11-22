# Miscellaneous

## Using external crates [rustc, cargo]
When using `rustc` you'll need to state where the external crate is
Or instead, you can use `cargo build` to build an unoptimised version of your project

Note: You must use `cargo-edit` from now to remove crates. i.e. `cargo rm` not `cargo uninstall`

## cargo binaries [cargo]
All are cached in `~/.cargo/bin` by default

## Naming Conventions [rust-lang]
https://rust-lang.github.io/api-guidelines/naming.html

## cargo [rust tool]
https://crates.io/

## cargo-edit [crate]
Tool to help you have the same behaviour as **npm**
```shell
cargo add <crate-name>
```

## References

- Cargo crate official repository - https://crates.io/
- Error installing a crate via cargo: specified package has no binaries - https://stackoverflow.com/questions/37706999/error-installing-a-crate-via-cargo-specified-package-has-no-binaries
- Can't find crate for `num` - https://stackoverflow.com/questions/29189615/cant-find-crate-for-num