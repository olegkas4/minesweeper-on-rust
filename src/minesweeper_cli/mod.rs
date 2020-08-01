use std::io;

use crate::minesweeper::{GridCell, Game};

fn index_to_char(i:u8) -> char {
    if i < 10 {
        ('0' as u8 + i) as char
    } else {
        ('A' as u8 + i - 10) as char
    }
}

fn char_to_index(c:char) -> usize {
    match c {
        '0'..='9' => c as usize - '0' as usize,
        'A'..='Z' => c as usize - 'A' as usize + 10,
        'a'..='z' => c as usize - 'a' as usize + 10,
        _ => 0
    }
}

fn draw_grid_cell(cell: &GridCell) {
    let neighbors_str = cell.count_neighbors().to_string();
    let ch = match (cell.is_hidden(), cell.is_flagged(), cell.is_mined(), cell.count_neighbors()){
        (true, true, _, _) => "F",
        (true, false, _, _) => "#",
        (false, _, true, _) => "*",
        (false, _, false, 0) => " ",
        (false, _, false, _) => neighbors_str.as_str()
    };
    print!("{:2}", ch);
}

fn draw_grid(game: &Game) {
    print!("{:2}", "");
    for c in 0..game.get_num_rows() {
        print!("{:2}", index_to_char(c as u8));
    }
    println!("");
    for (i, r) in game.get_rows().iter().enumerate() {
        print!("{:2}", index_to_char(i as u8));
        for c in r.iter() {
            draw_grid_cell(c);
        }
        println!("");
    }
    println!("--------------------------------------------------");
}

pub fn run(num_rows: usize, num_cols: usize, percent_mined: f32) -> io::Result<()> {
    let mut game = Game::new(num_rows, num_cols, percent_mined);
    loop {
        draw_grid(&game);

        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");
        
        if action.trim() == "Q" {
            println!("Goodbye");
            break;
        }
        let mut resp_chars = action.chars();
        let a = resp_chars.next().unwrap();
        let i = char_to_index(resp_chars.next().unwrap());
        let j = char_to_index(resp_chars.next().unwrap());
        println!("{:?} -> {:?}:{:?}", a, i, j);
        
        if a == 'F' {
            game.toggle_flag(i, j);
        } else if a == 'M' {
            game.dig(i, j);
        }

        if game.is_over() {
            draw_grid(&game);
            break
        }

    }
    
    println!("{}", if game.is_victory() {"YOU WON!"} else {"YOU LOOSE!"});
    Ok(())
}
