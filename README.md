togglog
-------
A toggle wrapper for the `log` crate.

Add togglog to your dependencies as if it was the log crate:
```
[dependencies]
log = { version = "0.1", package = "togglog" }
```
and enable it via features:
```
[features]
enable_log = ["log/enabled"]
```
then, in your code, use `log` macros normally:
```rust
if bad_thing {
    log::error!("bad thing!");
}
```

To compile out all logging, disable the `log/enabled` feature.

License
-------

This library may be used under your choice of the Apache 2.0 or MIT license.
