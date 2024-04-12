# ls-option: a tiny and simple to used library for ls-like functionality in pure Rust

## Quick Start

```zsh
# try to list all files with suffix `.rs` in current path
cargo run --example list_all_rs
# try to list paths in an unexisted path
cargo run --example list_unexisted_path
```

Example 1：
```rust
    use ls_option::*;
    let fs = ListOption::default()
        // allow to show file
        .file(true)
        // not allow to show dir
        .dir(false)
        // allow to show unhidden
        .unhidden(true)
        // not allow to show hidden
        .hidden(false)
        // walk recursively to search path
        .recursive(true)
        // only show path with one of these suffix
        .sufs(vec![".rs"])
        // start list the expected path search from `.`
        .list(".");
    dbg!(fs);
```

## License
[MIT](LICENSE)