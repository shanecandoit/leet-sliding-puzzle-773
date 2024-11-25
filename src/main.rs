
/* 773. Sliding Puzzle
On an 2 x 3 board, there are five tiles labeled from 1 to 5, and an empty square represented by 0.
A move consists of choosing 0 and a 4-directionally adjacent number and swapping it.

The state of the board is solved if and only if the board is [[1,2,3],[4,5,0]].

Given the puzzle board board, return the least number of moves required so that the state of the board is solved.
If it is impossible for the state of the board to be solved, return -1.

Example 1:
    Input: board = [[1,2,3],[4,0,5]]
    Output: 1
    Explanation: Swap the 0 and the 5 in one move.

Example 2:
    Input: board = [[1,2,3],[5,4,0]]
    Output: -1
    Explanation: No number of moves will make the board solved.

Example 3:
    Input: board = [[4,1,2],[5,0,3]]
    Output: 5
    Explanation: 5 is the smallest number of moves that solves the board.
    An example path:
    After move 0: [[4,1,2],[5,0,3]]
    After move 1: [[4,1,2],[0,5,3]]
    After move 2: [[0,1,2],[4,5,3]]
    After move 3: [[1,0,2],[4,5,3]]
    After move 4: [[1,2,0],[4,5,3]]
    After move 5: [[1,2,3],[4,5,0]]
*/

use std::collections::{HashSet, VecDeque};

struct Solution;

fn main() {
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

impl Solution {
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
