# Erase TS Types

## Building

Much of the build process isn't automated. Follow this guide (and try really hard).

1. `wasm-pack build --target nodejs`
2. Migrate changes to `//pkg` over to `//pkg-pure`. `//pkg-pure` is ESM only, which requires some tweaks in places. In general efforts are made to keep source identical.

## Testing

Tests exist in Rust and JavaScript to verify behaviour in all compilation modes.

### Rust

`cargo test`

### JS

TODO
