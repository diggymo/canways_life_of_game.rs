コンウェイのライフゲームをRust, Bevy, eguiで作ってみました。


```
cargo build --release --target wasm32-unknown-unknown 
wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/lifegame.wasm
```

