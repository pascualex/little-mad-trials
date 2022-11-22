# Little Mad Trials

## Hosting WASM locally

```
cargo run --release --target wasm32-unknown-unknown
```

## Building WASM

```
wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web .\target\wasm32-unknown-unknown\release\little-mad-trials.wasm
```
