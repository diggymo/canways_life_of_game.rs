set -e

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../blog/static/wasm-lifegame --target web ./target/wasm32-unknown-unknown/release/lifegame.wasm

