## Overview

This tiny crate provides the `var()` function which is the same as [`std::env::var`](https://doc.rust-lang.org/stable/std/env/fn.var.html) but the `NotPresent` variant of `VarError` provides the name of the missing environment variable:

```
pub enum VarError {
    NotPresent(String),
    NotUnicode(OsString),
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

