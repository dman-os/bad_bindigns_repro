# Binding mismatch repro

✅ [Solved](https://bytecodealliance.zulipchat.com/#narrow/channel/327223-wit-bindgen/topic/stitching.20together.20mutiple.20.60generate!.60.20sections/with/538242417): it was an issue with stable rust and it works in nightly.

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
