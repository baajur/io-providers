//! Defines traits and implementations for the inspection and manipulation of the process's
//! environment.

mod native;
mod simulated;

pub use self::native::NativeEnv;
pub use self::simulated::SimulatedEnv;

use std::env;
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};

/// Provides inspection and manipulation of the process's environment.
///
/// This roughly corresponds to [`std::env`](https://doc.rust-lang.org/std/env/).
///
/// # Examples
///
/// ```
/// extern crate io_providers;
///
/// use std::path::{Path, PathBuf};
/// use io_providers::{Env, NativeEnv, SimulatedEnv};
///
/// /// Uses `Env` to check if the currect working directory is "/foo/bar"
/// fn curdir_is_foobar<E: Env>(env: &mut E) -> bool {
///     let cur_dir = env.current_dir().unwrap();
///     cur_dir == PathBuf::from("/foo/bar")
/// }
///
/// fn main() {
///     // By creating a fake `Env` and set its current working directory, we can use it to test
///     // the behaviour of `curdir_is_foobar()`.
///     let mut env = SimulatedEnv::new();
///     env.set_current_dir(Path::new("/nope"));
///
///     // Test that our function returns false with a current working directory of "/nope"
///     assert!(!curdir_is_foobar(&mut env));
///
///     // Now set the fake working directory to "/foo/bar" and confirm that our function returns
///     // `true`
///     env.set_current_dir(Path::new("/foo/bar"));
///     assert!(curdir_is_foobar(&mut env));
///
///     // To use the real system environment, we use a `NativeEnv` instead
///     assert!(!curdir_is_foobar(&mut NativeEnv));
/// }
/// ```
pub trait Env {
    /// The iterator type returned by `args()`.
    type ArgsIter: Iterator<Item = String>;

    /// The iterator type returned by `args_os()`.
    type ArgsOsIter: Iterator<Item = ffi::OsString>;

    /// The iterator type returned by `vars()`.
    type VarsIter: Iterator<Item = (String, String)>;

    /// The iterator type returned by `vars_os()`.
    type VarsOsIter: Iterator<Item = (ffi::OsString, ffi::OsString)>;

    /// Returns the arguments which this program was started with (normally passed via the command
    /// line).
    ///
    /// See [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) for more information.
    fn args(&self) -> Self::ArgsIter;

    /// Returns the arguments which this program was started with (normally passed via the command
    /// line).
    ///
    /// See [`std::env::args_os`](https://doc.rust-lang.org/std/env/fn.args_os.html) for more
    /// information.
    fn args_os(&self) -> Self::ArgsOsIter;

    /// Returns the current working directory as a `PathBuf`.
    ///
    /// See [`std::env::current_dir`](https://doc.rust-lang.org/std/env/fn.current_dir.html) for
    /// more information.
    fn current_dir(&self) -> io::Result<PathBuf>;

    /// Returns the full filesystem path of the current running executable.
    ///
    /// See [`std::env::current_exe`](https://doc.rust-lang.org/std/env/fn.current_exe.html) for
    /// more information.
    fn current_exe(&self) -> io::Result<PathBuf>;

    /// Returns the path of the current user's home directory if known.
    ///
    /// See [`std::env::home_dir`](https://doc.rust-lang.org/std/env/fn.home_dir.html) for more
    /// information.
    #[deprecated(
        since = "0.2.0",
        note = "This function's behavior is unexpected and probably not what you want. \
                Consider using the home_dir function from crates.io/crates/dirs instead."
    )]
    fn home_dir(&self) -> Option<PathBuf>;

    /// Removes an environment variable from the environment of the currently running process.
    ///
    /// See [`std::env::remove_var`](https://doc.rust-lang.org/std/env/fn.remove_var.html) for more
    /// information.
    fn remove_var<K: AsRef<ffi::OsStr>>(&mut self, k: K);

    /// Changes the current working directory to the specified path, returning whether the change
    /// was completed successfully or not.
    ///
    /// See [`std::env::set_current_dir`](https://doc.rust-lang.org/std/env/fn.set_current_dir.html)
    /// for more information.
    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Sets the environment variable `k` to the value `v` for the currently running process.
    ///
    /// See [`std::env::set_var`](https://doc.rust-lang.org/std/env/fn.set_var.html) for more
    /// information.
    fn set_var<K: AsRef<ffi::OsStr>, V: AsRef<ffi::OsStr>>(&mut self, k: K, v: V);

    /// Returns the path of a temporary directory.
    ///
    /// See [`std::env::temp_dir`](https://doc.rust-lang.org/std/env/fn.temp_dir.html) for more
    /// information.
    fn temp_dir(&self) -> PathBuf;

    /// Fetches the environment variable `key` from the current process.
    ///
    /// See [`std::env::var`](https://doc.rust-lang.org/std/env/fn.var.html) for more information.
    fn var<K: AsRef<ffi::OsStr>>(&self, key: K) -> Result<String, env::VarError>;

    /// Fetches the environment variable `key` from the current process.
    ///
    /// See [`std::env::var_os`](https://doc.rust-lang.org/std/env/fn.var_os.html) for more information.
    fn var_os<K: AsRef<ffi::OsStr>>(&self, key: K) -> Option<ffi::OsString>;

    /// Returns an iterator of (variable, value) pairs of strings, for all the environment variables
    /// of the current process.
    ///
    /// See [`std::env::vars`](https://doc.rust-lang.org/std/env/fn.vars.html) for more information.
    fn vars(&self) -> Self::VarsIter;

    /// Returns an iterator of (variable, value) pairs of OS strings, for all the environment
    /// variables of the current process.
    ///
    /// See [`std::env::vars_os`](https://doc.rust-lang.org/std/env/fn.vars_os.html) for more information.
    fn vars_os(&self) -> Self::VarsOsIter;
}
