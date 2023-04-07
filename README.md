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
Error message is
```bash
wasmtime._trap.Trap: error while executing at wasm backtrace:
    0: 0x1884e6 - <unknown>!__rust_start_panic
    1: 0x18824f - <unknown>!rust_panic
    2: 0x188216 - <unknown>!std::panicking::rust_panic_with_hook::hd6f3df478dab5dc5
    3: 0x18728e - <unknown>!std::panicking::begin_panic_handler::{{closure}}::h7be61ca999d17a10
    4: 0x1871f0 - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::h1c7217a99e6903e8
    5: 0x187884 - <unknown>!rust_begin_unwind
    6: 0x18db3e - <unknown>!core::panicking::panic_fmt::h86214d64a55c1d57
    7: 0x18e0a7 - <unknown>!core::panicking::panic::h72f0442034a99972
    8: 0x9f30 - <unknown>!bindgen::bindgen::WasmtimePy::generate::h4d1cf13a2c876b5c
    9: 0x213ee - <unknown>!<bindgen::bindings::PythonBindings as bindgen::bindings::WasmtimePy>::generate::hc09c083e5b797d42
   10: 0x2160e - <unknown>!generate
   11: 0x19a4f6 - <unknown>!generate.command_export

Caused by:
    wasm trap: wasm `unreachable` instruction executed
```
