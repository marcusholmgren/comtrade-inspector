# comtrade inspector

You need to have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.

```bash
cargo install wasm-pack
```

Build the WASM module

```bash
cd comtrade_rust
wasm-pack build --target web
```

Run the application

```bash
cd app
npm run dev -- --open
```


## Reference

Rust [Comtrade library](https://github.com/drewsilcock/comtrade)
