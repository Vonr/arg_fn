# arg_fn

Argument parsing crate that allows the user to specify what to do for each argument.

# Example

```rs
#[derive(PartialEq, Debug, Default)]
struct Config {
    foo: bool,
    bar: bool,
}

let cfg = arg_fn::Parser::new(Config::default(), |_cfg, _arg| {})
    .arg("-foo", |cfg| cfg.foo = true)
    .arg("-nofoo", |cfg| cfg.foo = false)
    .arg("-bar", |cfg| cfg.bar = true)
    .arg("-nobar", |cfg| cfg.bar = false)
    .parse(["-bar", "-nofoo", "-foo", "-nobar", "-foo"]);

assert_eq!(
    cfg,
    Config {
        foo: true,
        bar: false,
    }
)
```
