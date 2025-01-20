# Étagère crash

It seems that the main example in Étagère's README crashes. This occurs with both `AtlasAllocator` and `BucketedAtlasAllocator`.

[Original repo](https://github.com/nical/etagere#example)

## Reproducing

```bash
cargo run
```

### Output

> thread 'main' panicked at src/main.rs:7:45:

> called `Option::unwrap()` on a `None` value

> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
