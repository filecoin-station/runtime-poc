# runtime-poc

Exploration of different JS/TS/WASM runtimes

## Deno + TypeScript

1. Install Deno (see [Installation docs](https://deno.land/manual@v1.29.2/getting_started/installation))

2. Run the following command:

   ```shell
   $ deno run --allow-net --allow-write --allow-read deno-typescript/demo.ts
   ``

## Deno + WASM + Rust

See https://deno.com/blog/wasmbuild and https://deno.land/x/wasmbuild@0.10.3

1. Install Deno (see [Installation docs](https://deno.land/manual@v1.29.2/getting_started/installation))

2. Build the demo

   ```shell
   $ (cd demo-wasm-rust && deno task wasmbuild)
   ```

3. Run the demo

  ```shell
  $ deno run --allow-read --allow-write --allow-net demo.ts
  ```

