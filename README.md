# minesweeper-on-rust

This project is an implementation of Minesweeper logic in Rust. It was created to be used in web application as WebAssembly based npm package
Check out: https://github.com/olegkas4/minesweeper-on-rust-wasm-ui


The actual game logic implementation is done in logic.rs which contains GridCell and Game classes. 
Another class MinesweeperBind is implemented in lib.rs and serves as an adapter between JS and logic implementations. It has all necessary wasm bindings applied and JS methods/libraries declarations like Math::random used to shuffle mines. 

The purpose of such 2 layers implementation is to keep actual logic implementation independent of wasm related stuff and reusable in other Rust UI projects. Trade off for this isolation is some code overhead