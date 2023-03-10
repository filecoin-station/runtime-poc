# runtime-poc

Exploration of different JS/TS/WASM runtimes

## Deno + TypeScript

1. Install Deno (see [Installation docs](https://deno.land/manual@v1.29.2/getting_started/installation))

2. Run the following command:

   ```shell
   $ deno run --allow-net --allow-write --allow-read deno-typescript/demo.ts
   ```

## Deno + WASM + Rust

See https://deno.com/blog/wasmbuild and https://deno.land/x/wasmbuild@0.10.3

1. Install Deno (see [Installation docs](https://deno.land/manual@v1.29.2/getting_started/installation))

2. Build the demo

   ```shell
   $ (cd deno-wasm-rust && deno task wasmbuild)
   ```

3. Run the demo

   ```shell
   $ (cd deno-wasm-rust && deno task demo)
   ```

## Wasmtime + Rust

0. Install cargo-wasi

   ```shell
   $ cargo install cargo-wasi
   ```

1. Build the WASM module

   ```shell
   $ (cd wasmtime/lib && cargo wasi build)
   ```

2. Build the demo runner

   ```shell
   $ (cd wasmtime && cargo build)
   ```

3. Run the demo

   ```shell
   $ ./wasmtime/target/debug/demo
   ```

## Wasmer + Rust

0. Install cargo-wasi

   ```shell
   $ cargo install cargo-wasi
   ```

1. Build the WASM module

   ```shell
   $ (cd wasmer/lib && cargo wasi build)
   ```

2. Build the demo runner

   ```shell
   $ (cd wasmer && cargo build)
   ```

3. Run the demo

   ```shell
   $ ./wasmer/target/debug/demo
   ```

## Wasmtime + Rust + async host functions

1. Build the WASM module

   ```shell
   $ (cd wasmtime-async-host/lib && cargo build --target=wasm32-unknown-unknown)
   ```

2. Build the demo runner

   ```shell
   $ (cd wasmtime-async-host && cargo build)
   ```

3. Run the demo

   ```shell
   $ ./wasmtime-async-host/target/debug/demo
   ```

## v8 + JavaScript

```shell
$ (cd v8-js && cargo run)
```

## Deno core

1. Install Deno (see [Installation docs](https://deno.land/manual@v1.29.2/getting_started/installation))

2. Build the demo module

   ```shell
   $ (cd deno-core && deno task wasmbuild)
   ```

3. Build & run the demo

   ```shell
   $ (cd deno-core && cargo run)
   ```

