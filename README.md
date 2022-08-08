 <div align="center">
   <a href="https://github.com/snowstry/snowstry">
    <img src="./frontend/assets/Snowstry.png" alt="Logo" width="20%">
   </a>

   <h3 align="center">Snowstry</h3>

   <p align="center">
   A chat app designed with simplicity and good user experience in mind.
   </p>
</div>

### Setup

**Dependencies**

-   [Baker](https://github.com/rv178/baker) optional, unless you wanna run commands yourself.
-   Yarn (for tailwindcss)
-   Rust
-   Trunk
-   Wasm target `wasm32-unknown-unknown`

```bash
bake setup
```

Install trunk and wasm-bindgen-cli:

```bash
cargo install trunk wasm-bindgen-cli
```

### Building

```bash
bake
```
