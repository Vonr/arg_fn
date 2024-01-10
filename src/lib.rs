//! Argument parsing crate that allows the user to specify what to do for each argument.
//!
//! # Example
//!
//! ```
//! #[derive(PartialEq, Debug, Default)]
//! struct Config {
//!     foo: bool,
//!     bar: bool,
//! }
//!
//! let cfg = arg_fn::Parser::new(Config::default(), |_cfg, _arg| {})
//!     .arg("-foo", |cfg| cfg.foo = true)
//!     .arg("-nofoo", |cfg| cfg.foo = false)
//!     .arg("-bar", |cfg| cfg.bar = true)
//!     .arg("-nobar", |cfg| cfg.bar = false)
//!     .parse(["-bar", "-nofoo", "-foo", "-nobar", "-foo"]);
//!
//! assert_eq!(
//!     cfg,
//!     Config {
//!         foo: true,
//!         bar: false,
//!     }
//! )
//! ```

use std::{borrow::Cow, collections::HashMap};

/// Parser struct containing the config, a map of arguments to functions, and a function that is
/// called when an argument is not in the map.
///
/// # Example
///
/// ```
/// #[derive(PartialEq, Debug, Default)]
/// struct Config {
///     foo: bool,
///     bar: bool,
/// }
///
/// let cfg = arg_fn::Parser::new(Config::default(), |_cfg, _arg| {})
///     .arg("-foo", |cfg| cfg.foo = true)
///     .arg("-nofoo", |cfg| cfg.foo = false)
///     .arg("-bar", |cfg| cfg.bar = true)
///     .arg("-nobar", |cfg| cfg.bar = false)
///     .parse(["-bar", "-nofoo", "-foo", "-nobar", "-foo"]);
///
/// assert_eq!(
///     cfg,
///     Config {
///         foo: true,
///         bar: false,
///     }
/// )
/// ```
#[allow(clippy::type_complexity)]
#[must_use]
pub struct Parser<'a, Config: 'a> {
    config: Config,
    arguments: HashMap<Cow<'a, str>, Box<dyn Fn(&mut Config) + 'a>>,
    unknown: Box<dyn Fn(&mut Config, &'a str) + 'a>,
}

impl<'a, Config: 'a> Parser<'a, Config> {
    pub fn new(config: Config, unknown: impl Fn(&mut Config, &'a str) + 'a) -> Self {
        Self {
            config,
            arguments: HashMap::new(),
            unknown: Box::new(unknown),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn with_arguments(
        config: Config,
        arguments: HashMap<Cow<'a, str>, Box<dyn Fn(&mut Config) + 'a>>,
        unknown: impl Fn(&mut Config, &'a str) + 'a,
    ) -> Self {
        Self {
            config,
            arguments,
            unknown: Box::new(unknown),
        }
    }

    pub fn arg(
        mut self,
        argument: impl Into<Cow<'a, str>>,
        callback: impl Fn(&mut Config) + 'a,
    ) -> Self {
        self.arguments.insert(argument.into(), Box::new(callback));
        self
    }

    pub fn parse(mut self, input: impl IntoIterator<Item = &'a str>) -> Config {
        for arg in input {
            if let Some(callback) = self.arguments.get(arg) {
                callback(&mut self.config);
            } else {
                (self.unknown)(&mut self.config, arg);
            };
        }

        self.config
    }
}

impl<'a, Config: Default> Default for Parser<'a, Config> {
    fn default() -> Self {
        Self::new(Config::default(), |_, _| {})
    }
}
