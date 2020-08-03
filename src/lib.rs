use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct GridCell {
    mined: bool,
    flagged: bool,
    hidden: bool,
    neighbors: u8,
    i: u16,
    j: u16,
}

impl GridCell {
    fn new(mined: bool, i: u16, j: u16) -> GridCell {
        GridCell {
            mined,
            flagged: false,
            hidden: true,
            neighbors: 0,
            i,
            j,
        }
    }

    pub fn is_flagged(&self) -> bool { self.flagged }
    pub fn is_hidden(&self) -> bool { self.hidden }
    pub fn is_mined(&self) -> bool { self.mined }
    pub fn get_neighbors_count(&self) -> u8 { self.neighbors }
}

pub struct Game {
    game_over: bool,
    victory: bool,
    num_rows: usize, 
    num_cols: usize, 
    num_total: usize,
    num_mined: usize,
    grid: Vec<Vec<GridCell>>,
    updated: Vec<(usize, usize)>
}

impl Game {

    pub fn get_num_rows(&self) -> usize {self.num_rows}
    pub fn get_num_cols(&self) -> usize {self.num_cols}
    pub fn is_over(&self) -> bool {self.game_over}
    pub fn is_victory(&self) -> bool {self.victory}
    pub fn get_cell_state(&self, i:usize, j:usize) -> (bool, bool, bool, u8) {
        let cell = &self.grid[i][j];
        (cell.hidden, cell.flagged, cell.mined, cell.neighbors)
    }
    
    fn count_neighbor(game: &mut Game, i:usize, j:usize, ni:usize, nj:usize) {
        if game.grid[ni as usize][nj as usize].mined {
            game.grid[i][j].neighbors += 1;
        }
    }
    
    fn dig_neighbor(game: &mut Game, i:usize, j:usize, ni:usize, nj:usize) {
        if game.grid[ni as usize][nj as usize].hidden {
            game.dig(ni as usize, nj as usize);
        }
    }
    
    fn do_around<F>(&mut self, i:usize, j:usize, action: &mut F)
        where F: FnMut(&mut Game, usize, usize, usize, usize) {
        let look_around: Vec<(i8, i8)> = vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];
        
        for (dr, dc) in &look_around {
            let (r, c) = (i as i8 + dr, j as i8 + dc);
            if r >= 0 && r < self.num_rows as i8
                && 0 <= c && c < self.num_cols as i8 {
                    action(self, i, j, r as usize, c as usize);
                }
        }
    }

    fn shuffle_mines(num_total: usize, num_mined: usize) -> Vec<bool> {
        let mut rng = thread_rng();
        let mut is_mined = [
            &vec![true; num_mined][..], 
            &vec![false; num_total-num_mined][..]].concat();
        is_mined.shuffle(&mut rng);
        
        is_mined
    }


    pub fn new(num_rows: usize, num_cols: usize, percent_mined: f32) -> Game {
        let num_total = num_rows*num_cols;
        let num_mined = ((num_rows * num_cols) as f32 * percent_mined) as usize;
        
        let is_mined = Game::shuffle_mines(num_total, num_mined);

        let mut grid = Vec::with_capacity(num_rows);
        for i in 0..num_rows {
            let mut row = Vec::with_capacity(num_cols);
            for j in 0..num_cols {
                let boom = is_mined[i * num_cols + j];
                row.push(GridCell::new(boom, i as u16, j as u16));
            }
            grid.push(row);
        }
    
        let mut game = Game {
            game_over: false,
            victory: false,
            num_rows, 
            num_cols, 
            num_total, 
            num_mined,
            grid,
            updated: Vec::with_capacity(num_total)
        };   

        for i in 0..num_rows {
            for j in 0..num_cols {
                game.do_around(i, j, &mut Game::count_neighbor)
            }
        }

        game
    }

    pub fn toggle_flag(&mut self, i: usize, j: usize)  -> &Vec<(usize, usize)> {
        self.updated.clear();
        if self.grid[i][j].hidden {
            self.grid[i][j].flagged = !self.grid[i][j].flagged;
            self.updated.push((i, j));
        }
        &self.updated
    }

    pub fn dig_cell(&mut self, i: usize, j: usize) -> &Vec<(usize, usize)> {
        self.updated.clear();
        self.dig(i, j);
        &self.updated
    }

    fn dig(&mut self, i: usize, j: usize){
        self.updated.push((i, j));
        self.grid[i][j].hidden = false;
        if self.grid[i][j].mined {
            self.game_over = true;
            self.victory = false;
        } else if self.grid[i][j].neighbors == 0 {
            self.do_around(i, j, &mut Game::dig_neighbor)
        }

        if self.check_won() {
            self.game_over = true;
            self.victory = true;
        }
    }

    pub fn get_left(&self) -> usize {
        let mut left = self.num_total;
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                if !self.grid[i][j].hidden && !self.grid[i][j].mined {
                    left -= 1
                }
            }
        }
        left - self.num_mined
    }
    
    fn check_won(&self) -> bool {
        self.get_left() == 0
    }
}
