# The Athena Engine

The Athena Engine is a chess engine written in Rust. 
The engine comes with a browser interface, written using html/css/js. 

## Rust Wasm Pack Template Instructions
This application was built using the Rust Wasm Pack Template. 

### How to install

```sh
npm install
```

### How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

### How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## Profiling

Need cargo packages; flamegraph. The following command will profile what is in the main.rs file.
```
cargo flamegraph
```

Had problems getting this to work on ubuntu. As a work around, need to install perf and the cargo inferno package. Sudo permissions needed.
```
sudo perf record --call-graph dwarf target/release/athena-engine
sudo perf script | inferno-collapse-perf > stacks.folded
cat stacks.folded | inferno-flamegraph > flamegraph.svg
```