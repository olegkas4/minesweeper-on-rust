mod utils;
mod logic;

use logic::Game;

use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
pub struct MinesweeperBind {
    game: Game
}

#[wasm_bindgen]
impl MinesweeperBind {
    pub fn new(num_rows: usize, num_cols: usize, percent_mined: f32) -> MinesweeperBind {
        MinesweeperBind {
            game: Game::new(num_rows, num_cols, percent_mined, Math::random)
        }
    }

    pub fn get_num_rows(&self) -> usize {self.game.get_num_rows()}
    pub fn get_num_cols(&self) -> usize {self.game.get_num_cols()}

    pub fn replay(&mut self) {
        self.game.replay();
    }

    pub fn cell_toggle_flag(&mut self, i: usize, j: usize) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        for (row, col) in self.game.toggle_flag(i, j) {
            res.push(*row);
            res.push(*col);
        }
        res
    }

    pub fn cell_dig(&mut self, i: usize, j: usize) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        for (row, col) in self.game.dig_cell(i, j) {
            res.push(*row);
            res.push(*col);
        }
        res
    }

    pub fn is_cell_flagged(&self, i: usize, j: usize) -> bool { 
        let (_, flagged, _, _) = self.game.get_cell_state(i, j);
        flagged 
    }

    pub fn is_cell_hidden(&self, i: usize, j: usize) -> bool {
        let (hidden, _, _, _) = self.game.get_cell_state(i, j);
        hidden 
    }
    
    pub fn is_cell_mined(&self, i: usize, j: usize) -> bool { 
        let (_, _, mined, _) = self.game.get_cell_state(i, j);
        mined
    }

    pub fn count_cell_neighbors(&self, i: usize, j: usize) -> u8 { 
        let (_, _, _, num_neighbors) = self.game.get_cell_state(i, j);
        num_neighbors
    }
    
    pub fn is_over(&self) -> bool { self.game.is_over() }
    pub fn is_victory(&self) -> bool { self.game.is_victory() }
    pub fn count_remaining_cells(&self) -> usize { self.game.count_remaining_cells() }
}
 