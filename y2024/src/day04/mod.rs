use std::ops::{Range};

#[derive(Debug,PartialEq, Clone)]
struct Move {
    x: isize,
    y: isize,
    word: String,
}

pub fn get_move(char_vec: &Vec<&str>, prev_word: String, x: isize, y: isize, bounds: (usize, usize)) -> Option<Move> {
    if(prev_word.len() >= 4 || x < 0 || y < 0 || x >= bounds.0 as isize || y >= bounds.1 as isize) {
        return None;
    }
    let uY = y as usize;
    let uX = x as usize;

    let new_move: Move = Move { x: x, y: y, word: prev_word + char_vec[uY].get(uX..uX+1).unwrap()};  

    return Some(new_move);
}

// Does a search from the starting spot and returns the number of XMASes formed
pub fn search(char_vec: &Vec<&str>, start: (isize,isize), bounds: (usize, usize)) -> i128 {
    let mut num_xmases = 0;
    
    let mut moves_vec: Vec<Option<Move>> = Vec::new();
    
    moves_vec.push(Some(Move { x: start.0, y: start.1, word:"X".to_string() }));

    while moves_vec.len() > 0 {
        let current = moves_vec.pop().unwrap();
        match current {
            None => continue,
            _ => {}
        }

        let new_current = current.unwrap();
        let current_word = new_current.word;
        if(current_word == "XMAS") {
            num_xmases += 1;
        }
        let x = new_current.x;
        let y = new_current.y;
        
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y-1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x-1, y+1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x, y+1, bounds));
        moves_vec.push(get_move(char_vec, current_word.clone(), x+1, y+1, bounds));
    }

    return num_xmases;
}

pub fn puzzle1(input: &str) -> i128 {
    let mut sum: i128 = 0;
    // Could just start at all the X's and DFS from there, with a maximum range of 4 moves.
    let char_vec: Vec<&str> = input.lines().collect();
    let bounds = (char_vec[0].len(), char_vec.len());
    
    for i in 0..bounds.1 {
        for j in 0..bounds.0 {
            if(char_vec[i].get(j..j+1).unwrap() == "X") {
                sum += search(&char_vec, (i as isize,j as isize), bounds);
            }
        }
    }

    return sum;
}

pub fn puzzle2(input: &str) -> i128 {
    let mut sum: i128 = 0;


    return sum;
}