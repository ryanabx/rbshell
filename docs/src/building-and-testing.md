## Building rbshell

Building is simple, just

```shell
cargo build --release
```

## Testing (nested compositor)

To test in a nested compositor, first open up the compositor of choice (for example, `cosmic-comp`)

```shell
cosmic-comp
```

then find the wayland display identifier (typically `wayland-1` for the first nested compositor session).

Set the `WAYLAND_DISPLAY` environment variable when running rbshell:

```shell
WAYLAND_DISPLAY=wayland-1 cargo run
```

> **NOTE:** rbshell has logging support! Just add `RUST_LOG=rbshell=[error|warn|info|debug|trace]` to your environment (select one of `error`, `warn`, `info`, `debug`, `trace`)