# Binding mismatch repro

Run: 
```bash
$ cargo build --target wasm32-wasip2
```

Expectation:
```
...
          Caused by:
              0: failed to decode world from module
              1: module was not valid
              2: failed to resolve import `wasmcloud:postgres/query@0.1.1-draft::[async-lower]query`
              3: failed to validate import interface `wasmcloud:postgres/query@0.1.1-draft`
              4: type mismatch for function `query`: expected `[I32, I32] -> [I32]` but found `[I32, I32, I32, I32, I32] -> [I32]`
...
```
