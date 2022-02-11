# Learning some Rust
## basics
Has some basic stuff

## canvasfun
Trying some wasm-bindgen stuff with canvas api.

First build rust code
```
cd canvasfun
cargo build
```

Then, install npm packages and run server
```
cd www
npm install
npm run start
```

Open second terminal, and watch the `wasm-pack build` command to continuously update the npm package for the javascript code.
```
cd canvasfun
cargo install cargo-watch
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"
```

