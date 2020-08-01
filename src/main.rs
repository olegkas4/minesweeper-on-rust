
const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 10;
const PERCENTAGE_MINED: f32 = 0.3;


use std::io;

mod minesweeper;
mod minesweeper_cli;


fn main() -> io::Result<()> {
    minesweeper_cli::run(NUM_ROWS, NUM_COLS, PERCENTAGE_MINED)
}