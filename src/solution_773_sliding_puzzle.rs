
use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    
pub fn main(){
    println!("773. Sliding Puzzle");

    // ex1
    let board = vec![vec![1,2,3], vec![4,0,5]];
    let result = Solution::sliding_puzzle(board);
    println!("Result: {}", result);
    assert_eq!(result, 1);

    // ex2
    let board = vec![vec![1,2,3], vec![5,4,0]];
    let result = Solution::sliding_puzzle(board);
    println!("Result: {}", result);
    assert_eq!(result, -1);

    // ex3
    let board = vec![vec![4,1,2], vec![5,0,3]];
    let result = Solution::sliding_puzzle(board);
    println!("Result: {}", result);
    assert_eq!(result, 5);
}


    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target = vec![vec![1,2,3], vec![4,5,0]];
        let target_str = Solution::puzzle_to_string(target.clone());

        let board_str = Solution::puzzle_to_string(board.clone());
        if board_str == target_str {
            return 0;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        queue.push_back((board, 0));
        visited.insert(board_str);
        
        while ! queue.is_empty() {
            let (current_board, moves) = queue.pop_front().unwrap();
            let current_board_str = Solution::puzzle_to_string(current_board.clone());

            if current_board_str == target_str {
                return moves;
            }

            let zero_pos = Solution::find_zero(&current_board);
            let (zero_x, zero_y) = zero_pos;

            for (dx, dy) in directions.iter() {
                let (new_x, new_y) = (zero_x as i32 + dx, zero_y as i32 + dy);
                if new_x < 0 || new_x >= 2 || new_y < 0 || new_y >= 3 {
                    continue;
                }

                let mut new_board = current_board.clone();
                new_board[zero_x][zero_y] = new_board[new_x as usize][new_y as usize];
                new_board[new_x as usize][new_y as usize] = 0;

                let new_board_str = Solution::puzzle_to_string(new_board.clone());
                if ! visited.contains(&new_board_str) {
                    queue.push_back((new_board, moves + 1));
                    visited.insert(new_board_str);
                }
            }
            
        }

        -1
    }

    fn puzzle_to_string(board: Vec<Vec<i32>>) -> String {
        let mut result = String::new();
        for row in board {
            for num in row {
                result.push_str(&num.to_string());
            }
        }
        result
    }

    fn find_zero(board: &Vec<Vec<i32>>) -> (usize, usize) {
        for i in 0..2 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }
}

// https://leetcode.com/problems/sliding-puzzle/submissions/1462549957/?envType=daily-question&envId=2024-11-25
// Runtime 6 ms
// Beats 100.00%
// Memory 2.22 MB
// Beats 50.00%
