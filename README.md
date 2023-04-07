# wasm-compositions

1. in *exporter* do `cargo build --target wasm32-unknown-unknown`
2. in *importer* do `cargo build --target wasm32-unknown-unknown`
3. to *target/wasm32-unknown-unknown/debug* copy `wasi_snapshot_preview1.wasm`
3. in *target/wasm32-unknown-unknown/debug* do all of the following:

```bash
# create a component from the importer, name it 'impcomp.wasm'
wasm-tools component new ./importer.wasm -o impcomp.wasm --adapt ./wasi_snapshot_preview1.wasm

# create a component from the exporter, name it 'excomp.wasm'
wasm-tools component new ./exporter.wasm -o excomp.wasm --adapt ./wasi_snapshot_preview1.wasm

# [optional] evaluate their interfaces
wasm-tools component wit impcomp.wasm
wasm-tools component wit excomp.wasm

# get the names right
cp excomp.wasm greeting.wasm

# compose
wasm-tools compose -o composed.wasm impcomp -p .
# output is 'composed component `composed.wasm`'

# [optional] evaluate their interfaces
wasm-tools component wit composed.wasm

# generate python bindings
# the first is just out of interest - generating bindings succeeds
python -m wasmtime.bindgen ./greeting.wasm --out-dir python/greeting/

# the second is the relevant binding - generating bindings fails
python -m wasmtime.bindgen ./composed.wasm --out-dir python/composition/
```

