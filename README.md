# minesweeper-on-rust

This project is an implementation of Minesweeper logic in Rust. It was created to be used in web application as WebAssembly based npm package
Check out: https://github.com/olegkas4/minesweeper-on-rust-wasm-ui


The actual game logic implementation is done in logic.rs which contains GridCell and Game classes. 
Another class MinesweeperBind is implemented in lib.rs and serves as an adapter or proxy between JS and logic implementations. It has all necessary wasm bindings applied and JS libraries and methods declarations, for example Math::random used to shuffle mines. 

The purpose of such 2 layers implementation is to keep actual logic implementation independent of wasm related stuff and reusable in other Rust UI projects. Trade off for this isolation is some code overhead


## MinesweeperBind usage

- MinesweeperBind(num_rows, num_cols, percent_mined) - creates an instance of the game
    - num_rows defines number of rows in new game
    - num_cols defines number of columns in new game
    - percent_mined - float value between 0 and 1 defining portion of mined cells

- get_num_rows() - returns number of rows in existing game

- get_num_cols() - returns number of columns in existing game

- replay() - restores same game to initial point to be replayed

- cell_toggle_flag(row, col) - function that sets or removes flag at cell
    - row - row index of a cell to flag
    - col - column index of cell to flag
    - returned value is a list of row and column indexes of cells that changed. row and column indexes should be read in pairs of array values, each even indexed value (0, 2, 4, 6, ...) represents row index, each odd indexed value (1, 3, 5, ...) represents column index, i.e. [cell0_row, cell0_col, cell1_row, cell1_col, ...]. For toggle flag in current implementation return list always has coordinates of only the toggled cell


- cell_dig(row, col) - function that uncovers/digs a cell
    - row - row index of a cell to dig
    - col - column index of cell to dig
    - returned value is a list of row and column indexes of cells that changed. row and column indexes should be read in pairs of array values, each even indexed value (0, 2, 4, 6, ...) represents row index, each odd indexed value (1, 3, 5, ...) represents column index, i.e. [cell0_row, cell0_col, cell1_row, cell1_col, ...]. It always returns cell been digged. In some cases it can return more then one cell, for example when mined cell is digged and game opens all cells or if digged cell has no mined cells around and game expands open cell area till reaches cells with mined neighbors.

- is_cell_flagged(row, col) - returns true if cell is currently flagged
    - row - row index of a cell to check
    - col - column index of cell to check

- is_cell_hidden(row, col) - returns true if cell was not yet digged/uncovered
    - row - row index of a cell to check
    - col - column index of cell to check

- is_cell_mined(row, col) - returns true if cell is mined
    - row - row index of a cell to check
    - col - column index of cell to check

- count_cell_neighbors(row, col) - returns number of mines surrounding a cell
    - row - row index of a cell to count mined neighbors
    - col - column index of cell to count mined neighbors

- is_over() - returns true if game is over with user win or lose

- is_victory() - returns true if user wins

- count_remaining_cells() - returns amount of cells yet to be uncovered
