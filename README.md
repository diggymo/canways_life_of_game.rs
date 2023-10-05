コンウェイのライフゲームをRust, Bevy, eguiで作ってみました。

![lifegame1](https://github.com/diggymo/canways_life_of_game.rs/assets/24623083/04328b30-019e-4747-b75f-8fb45b16d14b)

```
cargo build --release --target wasm32-unknown-unknown 
wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/lifegame.wasm
```
