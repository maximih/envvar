This tiny crate provides the `var()` function which is the same as [`std::env::var`](https://doc.rust-lang.org/stable/std/env/fn.var.html) but the `NotPresent` variant of `VarError` provides the name of the missing environment variable:

```
pub enum VarError {
    NotPresent(String),
    NotUnicode(OsString),
}
```

