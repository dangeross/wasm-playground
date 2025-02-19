[linux]
pack:
    wasm-pack build --weak-refs --target web --scope @breeztech

[macos]
pack:
    AR=/usr/local/opt/llvm/bin/llvm-ar CC=/usr/local/opt/llvm/bin/clang wasm-pack build --weak-refs --target web --scope @breeztech
