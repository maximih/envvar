use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::env;

/// Same as `std::env::VarError` but provides access to the name of the environment variable
/// which is `NotPresent`
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarError {
    /// The specified environment variable was not present in the current
    /// process's environment.
    NotPresent(String),
    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    NotUnicode(OsString),
}

impl fmt::Display for VarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VarError::NotPresent(ref s) => write!(f, "environment variable \"{:?}\" not found", s),
            VarError::NotUnicode(ref s) => {
                write!(f, "environment variable was not valid unicode: {:?}", s)
            }
        }
    }
}
 
impl Error for VarError {
    fn description(&self) -> &str {
        match *self {
            VarError::NotPresent(..) => "environment variable not found",
            VarError::NotUnicode(..) => "environment variable was not valid unicode",
        }
    }
}


/// Fetches the environment variable `key` from the current process.
///
/// The returned result is `Ok(s)` if the environment variable is present and is
/// valid unicode. If the environment variable is not present, or it is not
/// valid unicode, then `Err` will be returned.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "HOME";
/// match env::var(key) {
///     Ok(val) => println!("{}: {:?}", key, val),
///     Err(e) => println!("couldn't interpret {}: {}", key, e),
/// }
/// ```
pub fn var<K:AsRef<OsStr>>(key: K) -> Result<String, VarError> {
    match env::var_os(&key) {
        Some(s) => s.into_string().map_err(VarError::NotUnicode),
        None => Err(VarError::NotPresent(key.as_ref().to_str().unwrap().to_owned())),
    }
}

